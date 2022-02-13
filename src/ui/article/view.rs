use crate::{ui::article::content::ArticleContent, wiki::article::Article};

use cursive::{Vec2, View};

/// A view displaying an article
pub struct ArticleView {
    /// The content of the view
    content: ArticleContent,

    /// The last size the view had
    last_size: Vec2,
}

impl ArticleView {
    /// Creates a new ArticleView with a given article as its content
    pub fn new(article: Article) -> Self {
        log::debug!("creating a new instance of ArticleView");
        ArticleView {
            content: ArticleContent::new(article),
            last_size: Vec2::zero(),
        }
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &cursive::Printer) {
        // get the start and end y coordinates so that we only draw the lines visible
        let miny = printer.content_offset.y;
        let maxy = printer.content_offset.y + printer.output_size.y;

        // go through every line and print it to the screen
        for (y, line) in self
            .content
            .get_rendered_lines()
            .enumerate()
            .filter(|(y, _)| &miny <= y && y <= &maxy)
        {
            // go through every element in the line and print it with its style
            let mut x = 0;
            for element in line {
                printer.with_style(element.style, |printer| {
                    printer.print((x, y), &element.content);
                    x += element.width;
                });
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        // is this the same size as before? stop recalculating things!
        if self.last_size == size {
            return;
        }
        log::debug!("final size for the view is '({},{})'", size.x, size.y);

        // save the new size and compute the lines
        self.last_size = size;
        self.content.compute_lines(size);
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // calculate and return the required size
        log::debug!(
            "calculating the required size for '({},{})'",
            constraint.x,
            constraint.y
        );
        self.content.required_size(constraint)
    }

    fn take_focus(&mut self, _: cursive::direction::Direction) -> bool {
        // this view is always focusable
        true
    }
}
