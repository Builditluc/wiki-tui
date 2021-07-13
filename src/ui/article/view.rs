use crate::config::CONFIG;
use crate::wiki::article::*;
use cursive::align::Align;
use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::XY;
use cursive::{Printer, Vec2};

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
                ArticleElementType::Text => {
                    for (idx, content) in element.content.split("\n").enumerate() {
                        rendered_article.push(RenderedElement {
                            text: content.to_string(),
                            newline: if idx >= 1 { true } else { false },
                            style: Style::from(CONFIG.theme.text),
                            link_destination: element.link_target.clone(),
                        })
                    }
                }
                // if its a header, add some linebreaks and make the header bold
                ArticleElementType::Header => {
                    for (idx, content) in
                        format!("\n{}\n\n", element.content).split("\n").enumerate()
                    {
                        rendered_article.push(RenderedElement {
                            text: content.to_string(),
                            newline: if idx >= 1 { true } else { false },
                            style: Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                            link_destination: element.link_target.clone(),
                        })
                    }
                }
                // if its bold text, make it bold
                ArticleElementType::Bold => {
                    for (idx, content) in element.content.split("\n").enumerate() {
                        rendered_article.push(RenderedElement {
                            text: content.to_string(),
                            newline: if idx >= 1 { true } else { false },
                            style: Style::from(CONFIG.theme.text).combine(Effect::Bold),
                            link_destination: element.link_target.clone(),
                        })
                    }
                }
                // if its italic text, make it italic
                ArticleElementType::Italic => {
                    for (idx, content) in element.content.split("\n").enumerate() {
                        rendered_article.push(RenderedElement {
                            text: content.to_string(),
                            newline: if idx >= 1 { true } else { false },
                            style: Style::from(CONFIG.theme.text).combine(Effect::Italic),
                            link_destination: element.link_target.clone(),
                        })
                    }
                }
            }
        }

        self.elements_rendered = rendered_article;
    }

    fn calculate_lines(&mut self, max_width: usize) -> Vec<Line> {
        // TODO This needs some improvemenet

        // the width of the line that is currently calculated
        let mut line_width: usize = 0;

        let mut lines: Vec<Line> = Vec::new();
        let mut current_line: Vec<Element> = Vec::new();

        // go through every rendered element
        for element in self.elements_rendered.iter() {
            // does the element fit inside of the current line?
            let link_index = match element.link_destination {
                Some(ref destination) => Some(self.link_handler.push(Link {
                    position: (line_width, lines.len()).into(),
                    width: element.text.chars().count(),
                    destination: destination.to_string(),
                })),
                None => None,
            };

            if (line_width + element.text.chars().count()) < max_width && element.newline == false {
                current_line.push(Element {
                    text: element.text.to_string(),
                    style: element.style,
                    width: element.text.chars().count(),
                    link_index,
                });

                line_width += element.text.chars().count();
            } else {
                current_line.push(Element {
                    text: " ".repeat(max_width - line_width).to_string(),
                    style: Style::from(CONFIG.theme.text),
                    width: 0,
                    link_index: None,
                });

                line_width = 0;
                lines.push(std::mem::replace(
                    &mut current_line,
                    vec![{
                        Element {
                            text: element.text.to_string(),
                            style: element.style,
                            width: element.text.chars().count(),
                            link_index,
                        }
                    }],
                ));
            }
        }

        lines.push(current_line);
        lines
    }

    fn set_article(&mut self, article: Article) {
        // render the new article
        self.render(article);

        // after rendering, flush the caches
        self.historical_caches.clear();
        self.size_cache = None;
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

        self.content.lines = self.content.calculate_lines(size.x - 1);
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

    pub fn on_link_submit<F>(mut self, function: F) -> Self
    where
        F: 'static + Fn(&mut cursive::Cursive),
    {
        self.content.link_handler.on_link_submit_callback =
            cursive::event::Callback::from_fn(function);

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
        info!("[ui::article::AritlceView::required_size] Recalculating Size");
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

    fn important_area(&self, view_size: Vec2) -> cursive::Rect {
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
            //Event::Key(Key::Down) => self.move_current_link(Directions::DOWN),
            //Event::Key(Key::Up) => self.move_current_link(Directions::UP),
            Event::Key(Key::Enter) => EventResult::Consumed(Some(
                self.content.link_handler.on_link_submit_callback.clone(),
            )),
            _ => EventResult::Ignored,
        }
    }
}
