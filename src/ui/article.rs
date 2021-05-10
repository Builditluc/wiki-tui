use cursive::utils::lines::spans::*;
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::Printer;
use cursive::Vec2;

pub struct ArticleView {
    content: StyledString,
    rows: Vec<Row>,
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
        // got through every row and print it to the screen
        for (y, row) in self.rows.iter().enumerate() {
            let mut x = 0;
            for span in row.resolve(&self.content) {
                // print every span in a line with it's style and increase the x
                // value by the width of the span to prevent overwriting a previous span
                printer.with_style(*span.attr, |printer| {
                    printer.print((x, y), span.content);
                    x += span.width;
                })
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
}
