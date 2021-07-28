use crate::config::CONFIG;
use crate::wiki::article::*;
use cursive::align::Align;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::XY;
use cursive::{Printer, Vec2};
use std::rc;

use crate::ui::article::links::*;

pub struct ArticleView {
    content: ArticleContent,

    last_size: Vec2,
    width: Option<usize>,
}

struct ArticleContent {
    elements_rendered: Vec<RenderedElement>,
    elements_count: usize,

    lines: Vec<Line>,
    link_handler: LinkHandler,

    size_cache: Option<XY<SizeCache>>,
    historical_caches: Vec<(Vec2, Vec2)>,
}

#[derive(Debug)]
struct Element {
    text: String,
    style: Style,
    width: usize,
    link_index: Option<usize>,
}

struct RenderedElement {
    text: String,
    style: Style,
    newline: bool,
    link_destination: Option<String>,
}

type Line = Vec<Element>;

impl ArticleContent {
    pub fn new(content: StyledString) -> ArticleContent {
        let mut article_content = ArticleContent {
            elements_rendered: Vec::new(),
            elements_count: 0,

            lines: Vec::new(),
            link_handler: LinkHandler::new(),

            size_cache: None,
            historical_caches: Vec::new(),
        };

        // TODO do something better here
        article_content.render(Article {
            elements: vec![ArticleElement {
                content: content.source().to_string(),
                element_type: ArticleElementType::Text,
                link_target: None,
            }],
        });

        article_content
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
                    element.content.split("\n").enumerate(),
                    Style::from(CONFIG.theme.text),
                    &element.link_target,
                )),

                // if its a header, add some linebreaks and make the header bold
                ArticleElementType::Header => rendered_article.append(&mut self.render_element(
                    format!("\n{}\n\n", element.content).split("\n").enumerate(),
                    Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                    &element.link_target,
                )),
                // if its bold text, make it bold
                ArticleElementType::Bold => rendered_article.append(&mut self.render_element(
                    element.content.split("\n").enumerate(),
                    Style::from(CONFIG.theme.text).combine(Effect::Bold),
                    &element.link_target,
                )),
                // if its italic text, make it italic
                ArticleElementType::Italic => rendered_article.append(&mut self.render_element(
                    element.content.split("\n").enumerate(),
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
                newline: if idx >= 1 { true } else { false },
                style,
                link_destination: link_target.clone(),
            })
        }

        rendered_elements
    }

    fn calculate_lines(&mut self, max_width: usize) -> Vec<Line> {
        log::info!(
            "Calculating the lines with a max line width of: {}",
            max_width
        );

        // the width of the line that is currently calculated
        let mut line_width: usize = 0;

        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Vec<Element> = Vec::new();

        // go through every rendered element
        for (idx, element) in self.elements_rendered.iter().enumerate() {
            log::debug!("Rendering now the element no: {}", idx);
            let element_width = element.text.chars().count();
            let link_index = match element.link_destination {
                Some(ref destination) => Some(self.link_handler.push(Link {
                    position: (line_width, lines.len()).into(),
                    width: element.text.chars().count(),
                    destination: destination.to_string(),
                })),
                None => None,
            };
            log::trace!(
                "The element has the link_index: {:?}, and a width of: {}",
                link_index,
                element_width
            );

            // does the element fit in the current line?
            if (line_width + element_width) < max_width {
                // if it goes into a new line, add it to a new one
                if element.newline {
                    log::debug!("Adding the element to a new line");
                    // reset the line width
                    line_width = 0;

                    // fill the current line with empty spaces
                    current_line.push(Element {
                        text: " ".repeat(max_width - line_width).to_string(),
                        style: Style::from(CONFIG.theme.text),
                        width: max_width - line_width,
                        link_index: None,
                    });

                    // add the current line to the finished ones and add the new element to another
                    // line
                    self.add_element_to_new_line(
                        &mut lines,
                        &mut current_line,
                        element,
                        link_index,
                    );

                    // this element is finished, continue with the next one
                    continue;
                }

                // if the element goes to the current line, add it to the line
                log::debug!("Adding the element to the current line");
                current_line.push(self.create_element_from_rendered_element(element, link_index));

                // don't forget to increase the line width and continue with the next element
                line_width += element_width;
                log::trace!("New line width: {}", line_width);

                continue;
            } else {
                // hmm, the element doesn't fit into the current line
                // however, does the element go into a new line? if so, check if it fits into a
                // single line and split it if not
                log::debug!("The element no: {} doesn't fit into the current line", idx);
                if element.newline {
                    if element_width < max_width {
                        log::debug!("Adding the element to a new line");
                        // reset the line width
                        line_width = 0;

                        // fill the current line with empty spaces
                        current_line.push(Element {
                            text: " ".repeat(max_width - line_width).to_string(),
                            style: Style::from(CONFIG.theme.text),
                            width: max_width - line_width,
                            link_index: None,
                        });

                        // add the current line to the finished ones and add the new element to another
                        // line
                        self.add_element_to_new_line(
                            &mut lines,
                            &mut current_line,
                            element,
                            link_index,
                        );

                        // this element is finished, continue with the next one
                        continue;
                    }

                    // so.. the element goes into a new line, but it doesn't fit into a single one
                    // well, then split it and add it to a new line

                    log::debug!("The element no: {} must be splitted", idx);
                    let mut new_lines = self.split_element(element, link_index, 0, max_width);
                    log::trace!("splitted lines: \n{:?}", new_lines);

                    line_width = 0;

                    lines.push(std::mem::replace(&mut current_line, Vec::new()));
                    lines.append(&mut new_lines);
                    log::debug!("Added the current line and the new lines to the finished lines");

                    continue;
                }

                // split the element
                log::debug!("The element will now be splitted");
                let mut new_lines = self.split_element(element, link_index, line_width, max_width);
                log::trace!("splitted lines: \n{:?}", new_lines);

                // the first line of these new lines is merged with the current line
                current_line.append(&mut new_lines.remove(0));
                log::debug!("merged the first of these new lines with the current one");
                log::trace!("new_lines: \n{:?}", new_lines);
                log::trace!("new_lines after pop(): \n{:?}", new_lines.pop());
                lines.push(std::mem::replace(
                    &mut current_line,
                    new_lines.pop().unwrap_or_else(|| Vec::new()),
                ));
                log::debug!("added the other new lines to the finished ones");

                // all the other lines will be added to the finished lines
                lines.append(&mut new_lines);

                line_width = current_line.iter().map(|element| element.width).sum();
                log::debug!("new line width: {}", line_width);

                continue;
            }
        }

        // add the remaining line to the finished ones, because no elements are left
        lines.push(current_line);

        // return the finished lines
        lines
    }

    fn create_element_from_rendered_element(
        &self,
        element: &RenderedElement,
        link_index: Option<usize>,
    ) -> Element {
        Element {
            text: element.text.to_string(),
            style: element.style,
            width: element.text.chars().count(),
            link_index,
        }
    }

    fn split_element(
        &self,
        element: &RenderedElement,
        link_index: Option<usize>,
        line_width: usize,
        max_width: usize,
    ) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Line = Vec::new();
        let mut current_line_width = line_width;

        // first, split the element into chunks
        for (idx, chunk) in element.text.split(" ").enumerate().map(|(idx, chunk)| {
            if idx == 0 {
                (idx, chunk.to_string())
            } else {
                (idx, format!(" {}", chunk))
            }
        }) {
            log::debug!("the chunk no: {} will now be added", idx);
            let chunk_width = chunk.chars().count();
            log::trace!("chunk width: {}", chunk_width);

            // does the cunk fit into the current line? then add it and continue with the next one
            if (current_line_width + chunk_width) < max_width {
                current_line.push(Element {
                    text: chunk,
                    style: element.style,
                    width: chunk_width,
                    link_index,
                });
                log::debug!("Added the chunk to the current line");

                continue;
            }

            // if not, add it to a new line
            current_line_width = 0;
            let element_from_chunk = RenderedElement {
                text: chunk,
                style: element.style,
                newline: element.newline,
                link_destination: element.link_destination.clone(),
            };

            self.add_element_to_new_line(
                &mut lines,
                &mut current_line,
                &element_from_chunk,
                link_index,
            );
            log::debug!("Added the chunk no: {} to a new line", idx);
        }
        // add the remaining line to the finished ones, because no chunks are left
        lines.push(current_line);

        // return the finished lines
        lines
    }

    fn add_element_to_new_line(
        &self,
        lines: &mut Vec<Line>,
        current_line: &mut Line,
        element: &RenderedElement,
        link_index: Option<usize>,
    ) {
        log::trace!("current_line: \n{:?}", current_line);
        lines.push(std::mem::replace(
            current_line,
            vec![{
                let trimmed_element = RenderedElement {
                    text: element.text.trim_start().to_string(),
                    style: element.style,
                    newline: element.newline,
                    link_destination: element.link_destination.clone(),
                };
                self.create_element_from_rendered_element(&trimmed_element, link_index)
            }],
        ));
        log::trace!("new current line: \n{:?}", current_line);
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
    pub fn new<S>(content: S) -> ArticleView
    where
        S: Into<StyledString>,
    {
        ArticleView {
            content: ArticleContent::new(content.into()),
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

        self.content.lines = self.content.calculate_lines(size.x);
        self.width = self
            .content
            .lines
            .iter()
            .map(|line| line.iter().map(|element| element.width).max().unwrap_or(0))
            .max();
    }

    fn move_current_link(&mut self, direction: Directions) -> EventResult {
        self.content.link_handler.move_current_link(direction);
        EventResult::Consumed(None)
    }

    pub fn on_link_submit<F: Fn(&mut cursive::Cursive, &str) + 'static>(
        mut self,
        function: F,
    ) -> Self {
        self.content.link_handler.on_link_submit_callback = Some(rc::Rc::new(function));

        self
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let h = self.content.lines.len();

        let offset = Align::top_left().v.get_offset(h, printer.size.y);
        let printer = &printer.offset((0, offset));

        let miny = printer.content_offset.y;
        let maxy = printer.output_size.y + printer.content_offset.y;

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

        self.content
            .historical_caches
            .insert(0, (size, required_size));

        required_size
    }

    fn needs_relayout(&self) -> bool {
        self.content.size_cache.is_none()
    }

    fn important_area(&self, _view_size: Vec2) -> cursive::Rect {
        if self.content.link_handler.links.is_empty() {
            cursive::Rect::from((0, 0))
        } else {
            let link = &self.content.link_handler.links[self.content.link_handler.current_link];
            cursive::Rect::from_size(link.position, (link.width, 1))
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) => self.move_current_link(Directions::LEFT),
            Event::Key(Key::Right) => self.move_current_link(Directions::RIGHT),
            Event::Key(Key::Enter) => {
                let target = self.content.link_handler.links
                    [self.content.link_handler.current_link]
                    .destination
                    .clone();
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
