use crate::config::CONFIG;
use crate::wiki::article::Article;
use cursive::event::*;
use cursive::theme::Style;
use cursive::utils::lines::spans::*;
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::Printer;
use cursive::Vec2;

pub struct ArticleView {
    content: ArticleContent,
    selected: usize,
    width: usize,
}

pub struct ArticleContent {
    content: StyledString,
    lines: Vec<Vec<Element>>,
}

struct Element {
    content: String,
    style: Style,
    width: usize,
    selectable: bool,
}

impl ArticleContent {
    pub fn new(content: StyledString) -> ArticleContent {
        ArticleContent {
            content,
            lines: Vec::new(),
        }
    }

    fn calculate_rows(&mut self, size: Vec2) {
        self.lines.clear();

        // go through every row and convert the spans of the row to elements
        // and add them to the line
        for row in LinesIterator::new(&self.content, size.x) {
            let mut new_row: Vec<Element> = Vec::new();

            for span in row.resolve(&self.content) {
                new_row.push(Element {
                    content: span.content.to_string(),
                    style: *span.attr,
                    width: span.width,
                    selectable: false,
                })
            }
            self.lines.push(new_row);
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
            width: 0,
            selected: 0,
        }
    }

    // Replace the text in this view
    pub fn set_content<S>(&mut self, content: S)
    where
        S: Into<StyledString>,
    {
        self.content.content = content.into();
    }

    pub fn set_article(&mut self, article: Article) {
        self.set_content(article.content);
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let mut current_element = 0;

        // got through every row and print it to the screen
        for (y, line) in self.content.lines.iter().enumerate() {
            let mut x = 0;
            for element in line {
                // print every span in a line with it's style and increase the x
                // value by the width of the span to prevent overwriting a previous span
                let mut style = element.style;

                if current_element == self.selected && element.selectable {
                    style = style.combine(CONFIG.theme.highlight);
                }

                printer.with_style(style, |printer| {
                    printer.print((x, y), &element.content);
                    x += element.width;
                });

                current_element += 1;
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // set the new width and calculate the rows
        self.width = size.x;
        self.content.calculate_rows(size);
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        // calculate the rows with the given size and return the dimensions of the view
        self.content.calculate_rows(size);

        Vec2::new(self.width, self.content.lines.len())
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Key(Key::Left) && self.selected > 0 {
            self.selected -= 1;
            return EventResult::Consumed(None);
        }

        if event == Event::Key(Key::Right) {
            self.selected += 1;
            return EventResult::Consumed(None);
        }

        EventResult::Ignored
    }
}
