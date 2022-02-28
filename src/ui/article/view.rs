use crate::{config::CONFIG, ui::article::content::ArticleContent, wiki::article::Article};

use cursive::{
    direction::Absolute,
    event::{Callback, Event, EventResult, Key},
    Vec2, View,
};

use super::on_link_submit;

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
                let mut style = element.style;

                if Some(element.id) == self.content.current_link() {
                    style = style.combine(CONFIG.theme.highlight);
                }

                printer.with_style(style, |printer| {
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

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) if CONFIG.features.links => {
                self.content.move_selected_link(Absolute::Down, 1);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Right) if CONFIG.features.links => {
                self.content.move_selected_link(Absolute::Right, 1);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) if CONFIG.features.links => {
                log::info!("opening the link");

                // get current link and retrieve the ArticleElement linked to it
                let current_link = self.content.current_link();
                log::debug!("current link is '{:?}'", current_link);

                if let Some(element) = self.content.element_by_id(current_link) {
                    log::debug!("found the element");

                    // get target link from the article element
                    let target = match element.get_attribute("target") {
                        Some(t) => t.to_string(),
                        None => return EventResult::Ignored,
                    };
                    log::info!("target article is '{}'", target);

                    // return the callback
                    log::debug!("returning the callback to open the link");
                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                        on_link_submit(s, target.clone())
                    })));
                }

                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}
