use crate::config::CONFIG;
use crate::wiki::article::*;
use cursive::direction::Direction;
use cursive::event::*;
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::lines::spans::*;
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::{Printer, Vec2};

pub struct ArticleView {
    content: ArticleContent,
    width: usize,
}

struct ArticleContent {
    content: StyledString,
    lines: Vec<Row>,
    current: usize,
    elements_count: usize,
    last_size: Vec2,
}

impl ArticleContent {
    pub fn new(content: StyledString) -> ArticleContent {
        ArticleContent {
            content,
            lines: Vec::new(),
            current: 0,
            elements_count: 0,
            last_size: Vec2::max_value(),
        }
    }

    fn render(&mut self, article: Article) {
        self.elements_count = article.elements.len();
        let mut rendered_article = StyledString::new();

        // go trough every element in the article
        for element in article.elements.into_iter() {
            match element.element_type {
                // if its a link, make it underlined
                ArticleElementType::Link => {
                    let link_span = StyledString::styled(
                        element.content,
                        Style::from(CONFIG.theme.text).combine(Effect::Underline),
                    );

                    rendered_article.append(link_span);
                }
                // if its text, just append it to the rendered article
                ArticleElementType::Text => {
                    let text_span =
                        StyledString::styled(element.content, Style::from(CONFIG.theme.text));

                    rendered_article.append(text_span);
                }
                // if its a header, add some linebreaks and make the header bold
                ArticleElementType::Header => {
                    let header_span = StyledString::styled(
                        format!("\n{}\n\n", element.content),
                        Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                    );

                    rendered_article.append(header_span);
                }
            }
        }

        self.content = rendered_article;
    }

    fn change_current_element(&mut self, new_element: usize) {
        // go through every span in the content
        for (idx, span) in self.content.spans_raw_attr_mut().enumerate() {
            if idx == self.current {
                // remove the highlight for the previous selected span
                *span.attr = span.attr.combine(CONFIG.theme.text);
            } else if idx == new_element {
                // highlight the new selected span
                *span.attr = span.attr.combine(CONFIG.theme.highlight);
            }
        }
        self.current = new_element;
    }

    fn calculate_lines(&mut self, size: Vec2) {
        // calculate the lines with the given size
        self.lines = LinesIterator::new(&self.content, size.x).collect();
    }

    fn set_article(&mut self, article: Article) {
        // render the new article
        self.render(article);
    }
}

impl ArticleView {
    pub fn new<S>(content: S) -> ArticleView
    where
        S: Into<StyledString>,
    {
        ArticleView {
            content: ArticleContent::new(content.into()),
            width: 0,
        }
    }

    pub fn set_article(&mut self, article: Article) {
        self.content.set_article(article)
    }

    fn calculate_lines(&mut self, size: Vec2) {
        if self.content.last_size == size {
            return;
        }

        self.content.calculate_lines(size);

        self.width = if self.content.lines.iter().any(|line| line.is_wrapped) {
            size.x
        } else {
            self.content
                .lines
                .iter()
                .map(|line| line.width)
                .max()
                .unwrap_or(0)
        };

        self.content.last_size = size;
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let miny = printer.content_offset.y;
        let maxy = printer.output_size.y + printer.content_offset.y;

        info!("min: {}, max: {}", miny, maxy);
        // got through every row and print it to the screen
        for (y, line) in self.content.lines.iter().enumerate() {
            if y < miny || y >= maxy {
                if y >= maxy {
                    break;
                }
                warn!("Outside of bounds, y: {}", y);
                continue;
            }

            info!("Inside of bounds, y: {}", y);
            let mut x = 0;
            for span in line.resolve(&self.content.content) {
                // print every span in a line with it's style and increase the x
                // value by the width of the span to prevent overwriting a previous span
                printer.with_style(*span.attr, |printer| {
                    printer.print((x, y), &span.content);
                    x += span.width;
                });
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // set the new width and calculate the lines
        self.calculate_lines(size);
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        // calculate the lines with the given size and return the dimensions of the view
        self.calculate_lines(size);

        Vec2::new(self.width, self.content.lines.len())
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Key(Key::Left) && self.content.current > 0 {
            self.content
                .change_current_element(self.content.current - 1);
            return EventResult::Consumed(None);
        }

        if event == Event::Key(Key::Right) && self.content.current < self.content.elements_count {
            self.content
                .change_current_element(self.content.current + 1);
            return EventResult::Consumed(None);
        }

        EventResult::Ignored
    }
}
