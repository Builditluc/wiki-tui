use crate::config::CONFIG;
use cursive::event::*;
use cursive::utils::lines::spans::*;
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::Printer;
use cursive::Vec2;

pub struct ArticleView {
    content: StyledString,
    rows: Vec<Row>,
    selected: usize,
    width: usize,
}

impl ArticleView {
    pub fn new<S>(content: S) -> ArticleView
    where
        S: Into<StyledString>,
    {
        ArticleView {
            content: content.into(),
            rows: Vec::new(),
            width: 0,
            selected: 0,
        }
    }

    // Replace the text in this view
    pub fn set_content<S>(&mut self, content: S)
    where
        S: Into<StyledString>,
    {
        self.content = content.into();
    }

    fn calculate_rows(&mut self, size: Vec2) {
        // calculate the rows with a given size
        self.rows = LinesIterator::new(&self.content, size.x).collect();
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &Printer) {
        let mut current_element = 0;

        // got through every row and print it to the screen
        for (y, row) in self.rows.iter().enumerate() {
            let mut x = 0;
            for span in row.resolve(&self.content) {
                // print every span in a line with it's style and increase the x
                // value by the width of the span to prevent overwriting a previous span
                let mut style = *span.attr;

                if current_element == self.selected {
                    style = style.combine(CONFIG.theme.highlight);
                }

                printer.with_style(style, |printer| {
                    printer.print((x, y), span.content);
                    x += span.width;
                });

                current_element += 1;
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // set the new width and calculate the rows
        self.width = size.x;
        self.calculate_rows(size);
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        // calculate the rows with the given size and return the dimensions of the view
        self.calculate_rows(size);

        Vec2::new(self.width, self.rows.len())
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
