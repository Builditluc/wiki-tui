use crate::config::CONFIG;
use crate::wiki::article::*;
use cursive::align::Align;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::view::*;
use cursive::XY;
use cursive::{Printer, Vec2};
use std::cell::Cell;
use std::cmp::min;
use std::rc::Rc;

use crate::ui::article::lines::LinesWrapper;
use crate::ui::article::links::*;

pub struct ArticleView {
    content: ArticleContent,
    focus: Rc<Cell<usize>>,
    output_size: Cell<Vec2>,

    last_size: Vec2,
    width: Option<usize>,
}

struct ArticleContent {
    elements_rendered: Vec<RenderedElement>,
    elements_count: usize,

    lines: Vec<Line>,
    lines_wrapped: bool,
    link_handler: LinkHandler,

    headers: Vec<String>,
    headers_coords: Vec<usize>,

    size_cache: Option<XY<SizeCache>>,
    historical_caches: Vec<(Vec2, Vec2)>,
}

#[derive(Debug)]
pub struct Element {
    pub text: String,
    pub style: Style,
    pub width: usize,
    pub link_index: Option<usize>,
}

pub struct RenderedElement {
    pub text: String,
    pub style: Style,
    pub newline: bool,
    pub link_destination: Option<String>,
}

pub type Line = Vec<Element>;

impl ArticleContent {
    pub fn new() -> ArticleContent {
        ArticleContent {
            elements_rendered: Vec::new(),
            elements_count: 0,

            lines: Vec::new(),
            lines_wrapped: false,
            link_handler: LinkHandler::new(),

            headers: Vec::new(),
            headers_coords: Vec::new(),

            size_cache: None,
            historical_caches: Vec::new(),
        }
    }

    fn render(&mut self, article: Article) {
        self.elements_count = article.elements.len();
        let mut rendered_article = Vec::new();

        // go trough every element in the article
        for element in article.elements.into_iter() {
            match element.element_type {
                // if its a link, make it underlined
                ArticleElementType::Link => rendered_article.push(RenderedElement {
                    text: element.content,
                    newline: false,
                    style: Style::from(CONFIG.theme.text).combine(Effect::Underline),
                    link_destination: element.link_target,
                }),

                // if its text, just append it to the rendered article
                ArticleElementType::Text => rendered_article.append(&mut self.render_element(
                    element.content.split('\n').enumerate(),
                    Style::from(CONFIG.theme.text),
                    &element.link_target,
                )),

                // if its a header, add some linebreaks and make the header bold
                ArticleElementType::Header => {
                    rendered_article.append(&mut self.render_element(
                        format!("\n{}\n\n", element.content).split('\n').enumerate(),
                        Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                        &element.link_target,
                    ));
                    self.headers.push(element.content);
                }
                // if its bold text, make it bold
                ArticleElementType::Bold => rendered_article.append(&mut self.render_element(
                    element.content.split('\n').enumerate(),
                    Style::from(CONFIG.theme.text).combine(Effect::Bold),
                    &element.link_target,
                )),
                // if its italic text, make it italic
                ArticleElementType::Italic => rendered_article.append(&mut self.render_element(
                    element.content.split('\n').enumerate(),
                    Style::from(CONFIG.theme.text).combine(Effect::Italic),
                    &element.link_target,
                )),
            }
        }

        self.elements_rendered = rendered_article;
    }

    fn render_element<'a>(
        &self,
        element: impl Iterator<Item = (usize, &'a str)>,
        style: Style,
        link_target: &'a Option<String>,
    ) -> Vec<RenderedElement> {
        let mut rendered_elements: Vec<RenderedElement> = Vec::new();
        for (idx, content) in element {
            rendered_elements.push(RenderedElement {
                text: content.to_string(),
                newline: idx >= 1,
                style,
                link_destination: link_target.clone(),
            })
        }

        rendered_elements
    }

    fn calculate_lines(&mut self, max_width: usize) -> Vec<Line> {
        log::debug!(
            "Calculating the lines with a max line width of: {}",
            max_width
        );

        let lines_wrapper = LinesWrapper::new(max_width, self.headers.clone())
            .calculate_lines(&self.elements_rendered, &mut self.link_handler);

        self.lines_wrapped = lines_wrapper.lines_wrapped;

        self.headers = lines_wrapper.headers;
        self.headers_coords = lines_wrapper.header_coords;

        return lines_wrapper.lines;
    }

    fn set_article(&mut self, article: Article) {
        // render the new article
        self.render(article);

        // after rendering, flush the caches
        self.historical_caches.clear();
        self.size_cache = None;

        // also, reset the link_handler
        self.link_handler.reset();
    }

    fn is_chache_valid(&self, size: Vec2) -> bool {
        match self.size_cache {
            None => false,
            Some(ref last) => last.x.accept(size.x) && last.y.accept(size.y),
        }
    }
}

impl ArticleView {
    pub fn new() -> ArticleView {
        ArticleView {
            content: ArticleContent::new(),
            focus: Rc::new(Cell::new(0)),
            output_size: Cell::new(Vec2::zero()),
            last_size: Vec2::zero(),
            width: None,
        }
    }

    pub fn set_article(&mut self, article: Article) {
        self.content.set_article(article)
    }

    fn calculate_lines(&mut self, size: Vec2) {
        if self.content.is_chache_valid(size) || size.x == 0 {
            return;
        }

        self.content.size_cache = None;
        self.content.link_handler.links.clear();

        self.content.lines_wrapped = false;
        self.content.lines = self.content.calculate_lines(size.x);

        // Desired width
        self.width = if self.content.lines_wrapped {
            log::debug!("Rows are wrapped, requiring the full width of '{}'", size.x);
            // if any rows are wrapped, then require the full width
            Some(size.x)
        } else {
            log::debug!("Rows aren't wrapped");
            let size_x = self
                .content
                .lines
                .iter()
                .map(|line| line.iter().map(|element| element.width).max().unwrap_or(0))
                .max();

            size_x
        }
    }

    fn move_link(&mut self, direction: Directions) -> EventResult {
        let link_pos_y = self.content.link_handler.move_current_link(direction);
        if link_pos_y < self.focus.get() {
            self.move_focus_up(self.focus.get().saturating_sub(link_pos_y));
        } else if (self.output_size.get().y + self.focus.get()) < link_pos_y {
            self.move_focus_down(
                link_pos_y.saturating_sub(self.output_size.get().y + self.focus.get()),
            );
        }
        EventResult::Consumed(None)
    }

    pub fn on_link_submit<F: Fn(&mut cursive::Cursive, &str) + 'static>(
        mut self,
        function: F,
    ) -> Self {
        self.content.link_handler.on_link_submit_callback = Some(Rc::new(function));

        self
    }

    fn move_focus_up(&mut self, n: usize) -> EventResult {
        let focus = self.focus.get().saturating_sub(n);
        self.focus.set(focus);
        let link_pos_y = self.content.link_handler.links[self.content.link_handler.current_link]
            .position
            .y;
        if self.output_size.get().y < link_pos_y {
            self.content.link_handler.move_current_link(Directions::UP);
        }
        EventResult::Consumed(None)
    }

    fn move_focus_down(&mut self, n: usize) -> EventResult {
        let focus = min(
            self.focus.get() + n,
            self.content.lines.len().saturating_sub(1),
        );
        self.focus.set(focus);
        let link_pos_y = self.content.link_handler.links[self.content.link_handler.current_link]
            .position
            .y;
        if self.focus.get() > link_pos_y {
            self.content
                .link_handler
                .move_current_link(Directions::DOWN);
        }
        EventResult::Consumed(None)
    }

    pub fn select_header(&mut self, header: usize) {
        if (header >= self.content.headers_coords.len()) || (header >= self.content.headers.len()) {
            log::error!("The Header could not be found");
            return;
        }

        let header_pos = self.content.headers_coords[header];
        let focus = self.focus.get();

        log::debug!("header_pos: {}, focus: {}", header_pos, focus);

        if header_pos > focus {
            self.move_focus_down(header_pos.saturating_sub(focus));
        } else {
            self.move_focus_up(focus.saturating_sub(header_pos));
        }

        log::debug!("current focus: {}", self.focus.get());
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let h = self.content.lines.len();

        let offset = Align::top_left().v.get_offset(h, printer.size.y);
        let printer = &printer.offset((0, offset));

        let miny = printer.content_offset.y;
        let maxy = printer.output_size.y + printer.content_offset.y;

        self.output_size.set(printer.output_size);

        // got through every row and print it to the screen
        for (y, line) in self.content.lines.iter().enumerate() {
            if y < miny || y >= maxy {
                continue;
            }

            let l = line.iter().map(|element| element.width).max().unwrap_or(0);
            let mut x = Align::top_left().h.get_offset(l, printer.size.x);

            for span in line {
                let style = if span.link_index.unwrap_or(999999)
                    == self.content.link_handler.current_link
                {
                    span.style.combine(CONFIG.theme.highlight)
                } else {
                    span.style
                };
                printer.with_style(style, |printer| {
                    printer.print((x, y), &span.text);
                    x += span.width;
                });
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // is this the same size as before? stop recalculating things!
        if self.last_size == size {
            return;
        }

        // set the new width and calculate the lines
        self.last_size = size;
        self.calculate_lines(size);

        let my_size = Vec2::new(self.width.unwrap_or(0), self.content.lines.len());
        self.content.size_cache = Some(SizeCache::build(my_size, size));
        self.content.historical_caches.clear();

        log::debug!("size of view: {:?}", size);
        log::trace!("view width is: '{}'", size.x);
        log::trace!("lines: \n{:#?}", self.content.lines);
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        // do we already have the required size calculated and cached?
        for previous_size in self.content.historical_caches.iter() {
            let req_size = previous_size.0;
            if req_size == size {
                return previous_size.1;
            }
        }

        // if we don't have the size calculated, calculate it and add it to the cache
        self.calculate_lines(size);
        let required_size = Vec2::new(self.width.unwrap_or(0), self.content.lines.len());

        log::debug!(
            "The required size for '{:?}' is '{:?}'",
            size,
            required_size
        );
        self.content
            .historical_caches
            .insert(0, (size, required_size));

        required_size
    }

    fn needs_relayout(&self) -> bool {
        self.content.size_cache.is_none()
    }

    fn important_area(&self, _: Vec2) -> cursive::Rect {
        Some(self.focus.get())
            .map(|i| {
                cursive::Rect::from_size(
                    (0, i),
                    (self.output_size.get().x, self.output_size.get().y),
                )
            })
            .unwrap_or_else(|| cursive::Rect::from((0, 1)))
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) => self.move_link(Directions::LEFT),
            Event::Key(Key::Right) => self.move_link(Directions::RIGHT),
            Event::Key(Key::Up) => self.move_focus_up(1),
            Event::Key(Key::Down) => self.move_focus_down(1),
            Event::Key(Key::Enter) => {
                let target = self.content.link_handler.links
                    [self.content.link_handler.current_link]
                    .destination
                    .clone();
                log::info!("Showing the dialog to open '{}'", target);
                EventResult::Consumed(
                    self.content
                        .link_handler
                        .on_link_submit_callback
                        .clone()
                        .map(|f| Callback::from_fn(move |s| f(s, &target))),
                )
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Default for ArticleView {
    fn default() -> Self {
        Self::new()
    }
}
