use crate::{
    config::CONFIG,
    ui::article::content::ArticleContent,
    ui::{article::on_link_submit, utils::display_message},
    wiki::article::Article,
};

use cursive::{
    direction::Absolute,
    event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent},
    view::CannotFocus,
    Rect, Vec2, View,
};

use std::cell::Cell;

/// A view displaying an article
pub struct ArticleView {
    /// The content of the view
    content: ArticleContent,

    /// The last size the view had
    last_size: Vec2,

    /// The offset of the viewport
    viewport_offset: Cell<usize>,

    /// The size of the viewport
    viewport_size: Cell<Vec2>,
}

impl ArticleView {
    /// Creates a new ArticleView with a given article as its content
    pub fn new(article: Article) -> Self {
        debug!("creating a new instance of ArticleView");
        ArticleView {
            content: ArticleContent::new(article),
            last_size: Vec2::zero(),
            viewport_offset: Cell::new(0),
            viewport_size: Cell::new(Vec2::zero()),
        }
    }

    /// Moves the viewport by a given amount in a given direction
    fn scroll(&mut self, direction: Absolute, amount: usize) -> EventResult {
        debug!("scrolling '{:?}' with an amount of '{}'", direction, amount);
        match direction {
            Absolute::Up => self
                .viewport_offset
                .set(self.viewport_offset.get().saturating_sub(amount)),
            Absolute::Down => self
                .viewport_offset
                .set(self.viewport_offset.get().saturating_add(amount)),
            _ => return EventResult::Ignored,
        }

        // if the links are enabled, check if the current link is out of the viewport
        if !CONFIG.features.links {
            return EventResult::Consumed(None);
        }

        // get the position of the current link and the top of the viewport
        let link_pos = self.content.current_link_pos().unwrap_or_default();
        let viewport_top = self.viewport_offset.get();

        // if the link is above the viewport (aka its y-pos is smaller than the viewport offset),
        // then increase the links position by the difference between the viewport offset and its
        // y-position
        if link_pos.y <= viewport_top {
            let move_amount = viewport_top.saturating_sub(link_pos.y);
            self.content.move_selected_link(Absolute::Down, move_amount);

            return EventResult::Consumed(None);
        }

        // if the link is below the viewport (aka its y-pos is bigger than the viewport offset plus
        // its size),
        // then decrease the links position by the difference between its y-position and the
        // viewport offset
        let viewport_bottom = viewport_top.saturating_add(self.viewport_size.get().y);
        if link_pos.y >= viewport_bottom {
            let move_amount = link_pos.y.saturating_sub(viewport_bottom);
            self.content.move_selected_link(Absolute::Up, move_amount);

            return EventResult::Consumed(None);
        }

        EventResult::Consumed(None)
    }

    /// Select a header by moving the viewport to its coordinates
    pub fn select_header(&mut self, index: usize) {
        if !CONFIG.features.toc {
            return;
        }
        info!("selecting the header '{}'", index);

        // get the position of the header and the viewport top and bottom
        let header_pos = self
            .content
            .header_y_pos(index)
            .unwrap_or_else(|| self.viewport_offset.get());
        let viewport_top = self.viewport_offset.get();

        debug!(
            "header_pos: '{}' viewport_top: '{}'",
            header_pos, viewport_top
        );

        // if the header is above the viewport, then get the difference between the header and the
        // viewport and scroll up by that amount
        if header_pos < viewport_top {
            let move_amount = viewport_top.saturating_sub(header_pos);
            self.scroll(Absolute::Up, move_amount);
            debug!("scrolled '{}' up", move_amount);
            return;
        }

        // if the header is below the viewport, then get the difference between the header and the
        // viewport and scroll down by that amount
        let move_amount = header_pos.saturating_sub(viewport_top);
        self.scroll(Absolute::Down, move_amount);
        debug!("scrolled '{}' down", move_amount);
    }

    /// Check if the link can safely be opened and open it
    fn check_and_open_link(&self) -> EventResult {
        // get current link and retrieve the ArticleElement linked to it
        let current_link = self.content.current_link();
        debug!("current link is '{:?}'", current_link);

        if let Some(element) = self.content.element_by_id(current_link) {
            debug!("found the element of the link");

            // get target link from the article element
            let target = match element.get_attribute("target") {
                Some(t) => t.to_string(),
                None => {
                    warn!("missing attribute 'target' from element '{}'", element.id());
                    warn!("the link '{}' is not valid", element.id());
                    return EventResult::Ignored;
                }
            };
            debug!("target link is '{}'", target);

            // check whether this link pointing to another wikipedia article
            if element.get_attribute("external").is_some() {
                warn!("element '{}' contains attribute 'external'", element.id());
                warn!("the link '{}' is external", element.id());
                return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                    let title = "Information";
                    let message = format!("This link doesn't point to another article. \nInstead, it leads to the following external webpage and therefore, cannot be opened: \n\n> {}", target);
                    display_message(s, title, &message);
                })));
            }
            debug!("target link is pointing to another wikipedia article");

            // return the callback
            debug!("returning the callback to open the link");
            info!(
                "opening the link '{}' with the target '{}'",
                element.id(),
                target
            );
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                on_link_submit(s, target.clone())
            })));
        }

        return EventResult::Ignored;
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &cursive::Printer) {
        // get the start and end y coordinates so that we only draw the lines visible
        let miny = printer.content_offset.y;
        let maxy = printer.content_offset.y + printer.output_size.y;

        // update the viewport
        self.viewport_offset.set(miny);
        self.viewport_size.set(printer.output_size);

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
        debug!("final size for the view is '({},{})'", size.x, size.y);

        // save the new size and compute the lines
        self.last_size = size;
        self.content.compute_lines(size);
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // calculate and return the required size
        self.content.required_size(constraint)
    }

    fn take_focus(&mut self, _: cursive::direction::Direction) -> Result<EventResult, CannotFocus> {
        // this view is always focusable
        Ok(EventResult::Consumed(None))
    }

    fn important_area(&self, _: Vec2) -> cursive::Rect {
        // return the viewport
        Rect::from_size(
            Vec2::new(0, self.viewport_offset.get()),
            self.viewport_size.get(),
        )
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => self.scroll(Absolute::Up, 1),
            Event::Key(Key::Down) => self.scroll(Absolute::Down, 1),
            Event::Key(Key::Left) if CONFIG.features.links => {
                self.content.move_selected_link(Absolute::Left, 1);
                // if the current link is outside of the viewport, then scroll
                // get the current links position
                let current_link_pos = self
                    .content
                    .current_link_pos()
                    .unwrap_or_else(|| (0, 0).into());

                // we've moved the link to the left, so we only need to check if the link is above
                // the viewport
                let viewport_top = self.viewport_offset.get();
                if current_link_pos.y <= viewport_top {
                    // so the link is below the viewport... great...
                    // calculate how much below the viewport the link is
                    let move_amount = viewport_top.saturating_sub(current_link_pos.y);

                    // then scroll that amount
                    self.scroll(Absolute::Up, move_amount);
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Right) if CONFIG.features.links => {
                self.content.move_selected_link(Absolute::Right, 1);
                // if the current link is outside of the viewport, then scroll
                // get the current links position
                let current_link_pos = self
                    .content
                    .current_link_pos()
                    .unwrap_or_else(|| (0, 0).into());

                // we've moved the link to the right, so we only need to check if the link is below
                // the viewport
                let viewport_bottom = self
                    .viewport_offset
                    .get()
                    .saturating_add(self.viewport_size.get().y);
                if current_link_pos.y >= viewport_bottom {
                    // so the link is below the viewport... great...
                    // calculate how much below the viewport the link is
                    let move_amount = current_link_pos.y.saturating_sub(viewport_bottom);

                    // then scroll that amount
                    self.scroll(Absolute::Down, move_amount);
                }

                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) if CONFIG.features.links => self.check_and_open_link(),
            Event::Mouse {
                event: MouseEvent::Release(MouseButton::Left),
                position,
                offset,
            } => {
                // get what element was clicked
                if let Some(element) = self
                    .content
                    .get_element_at_position(position.saturating_sub(offset))
                {
                    return match element.get_attribute("type") {
                        // if it's a link, check if it's valid and then open it
                        Some("link") if CONFIG.features.links => {
                            // select this link
                            let element_id = *element.id();
                            self.content.set_current_link(element_id);
                            debug!("selected the clicked link");

                            self.check_and_open_link()
                        }

                        // if it's a button, don't do anything for now
                        Some("button") => {
                            error!("wow, you've found a secret!");
                            EventResult::Ignored
                        }

                        // this element doesn't support mouse clicking
                        _ => EventResult::Ignored,
                    };
                }

                // if there isn't an element at the event position, ignore the event altogether
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}
