use crate::{
    config::CONFIG,
    ui::{
        article::{content::ArticleContent, on_link_submit},
        scroll::{important_area, on_event},
        utils::display_message,
    },
    wiki::article::{Article, ElementType},
};

use cursive::{
    event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent},
    view::{
        scroll::{self, Core},
        CannotFocus,
    },
    Rect, Vec2, View,
};

/// A view displaying an article
pub struct ArticleView {
    /// The content of the view
    content: ArticleContent,

    scroll: Core,

    last_size: Vec2,
}
impl ArticleView {
    /// Creates a new ArticleView with a given article as its content
    pub fn new(article: Article) -> Self {
        debug!("creating a new instance of ArticleView");
        ArticleView {
            content: ArticleContent::new(article),
            scroll: Core::new(),
            last_size: Vec2::zero(),
        }
    }

    pub fn select_anchor(&mut self, anchor: &str) {
        if let Some(anchor_coord) = self.content.anchor(anchor) {
            self.scroll.set_offset((0, anchor_coord));
        }
    }

    /// Checks if the current link is out of the viewport and moves the selection accordingly. If
    /// no links could be found in the current viewport, the selection stays as it was
    fn check_and_update_selection(&mut self) {
        if !self.content.has_links() {
            return;
        }

        let selection_coords = self.content.current_link_coords();
        let viewport = self.scroll.content_viewport();

        if viewport.contains(selection_coords) {
            return;
        }

        if selection_coords.y < viewport.top() {
            let (id, _) = self
                .content
                .links()
                .skip(self.content.current_link_idx())
                .find(|(_, pos)| viewport.contains(*pos))
                .map(|x| x.to_owned())
                .unwrap_or((self.content.current_link_element_id(), Vec2::zero()));
            self.content.select_link_by_id(id);
            return;
        }
        if selection_coords.y > viewport.bottom() {
            let (id, _) = self
                .content
                .links()
                .rev()
                .find(|(_, pos)| viewport.contains(*pos))
                .map(|x| x.to_owned())
                .unwrap_or((self.content.current_link_element_id(), Vec2::zero()));
            self.content.select_link_by_id(id);
        }
    }

    /// Checks if the current viewport shows the selected link and if not, moves it so the link is
    /// visible
    fn check_and_update_viewport(&mut self) {
        if !self.content.has_links() {
            return;
        }

        let selection_coords = self.content.current_link_coords();
        self.scroll.scroll_to_y(selection_coords.y);
        self.scroll.scroll_to_x(selection_coords.x);
    }

    /// Check if the link can be opened and opens it if it can
    fn check_and_open_link(&self) -> EventResult {
        if let Some(element) = self
            .content
            .element_by_id(self.content.current_link_element_id())
        {
            debug!("found the element of the link");

            // get the target link from the element
            let target = match element.attr("target") {
                Some(t) => t.to_string(),
                None => {
                    warn!("missing attribute 'target' from element '{}'", element.id());
                    warn!("the link '{}' is not valid", element.id());
                    return EventResult::Ignored;
                }
            };
            debug!("target is '{}'", target);

            // check whether the link is pointing to another wikipedia article or if its external
            if element.attr("external").is_some() {
                warn!("element '{}' contains attribute 'external'", element.id());
                warn!("the link '{}' is external", element.id());
                return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                    display_message(
                        s, "Information", &format!("This link doensn't point to another article. \nInstead, it leads to the following external webpage and therefore, cannot be opened: \n\n> {}", target)
                    );
                })));
            }
            debug!("target link is not external, continuing");
            info!(
                "opening the link '{}' with the target '{}'",
                element.id(),
                target
            );
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                on_link_submit(s, target.clone())
            })));
        }
        EventResult::Ignored
    }
}

impl View for ArticleView {
    fn draw(&self, printer: &cursive::Printer) {
        let printer = self.scroll.sub_printer(printer);
        let viewport = self.scroll.content_viewport();

        // go through every line and print it to the screen
        for (y, line) in self
            .content
            .get_rendered_lines()
            .enumerate()
            .filter(|(y, _)| &viewport.top() <= y && y <= &viewport.bottom())
        {
            // go through every element in the line and print it with its style
            let mut x = 0;
            for element in line {
                let mut style = element.style;
                if element.id == self.content.current_link_element_id() && CONFIG.features.links {
                    style = style.combine(CONFIG.theme.highlight)
                }
                printer.with_style(style, |printer| {
                    printer.print((x, y), &element.content);
                    x += element.width;
                });
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        if self.last_size == size {
            return;
        }

        self.content.compute_lines(size);
        scroll::layout(
            self,
            size,
            self.needs_relayout(),
            |s, size| s.content.compute_lines(size),
            |s, constraint| s.content.required_size(constraint),
        );
        self.last_size = size;
        debug!("final size for the view is '({},{})'", size.x, size.y);
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // calculate and return the required size
        self.content.required_size(constraint)
    }

    fn needs_relayout(&self) -> bool {
        self.scroll.needs_relayout()
    }

    fn important_area(&self, view_size: Vec2) -> cursive::Rect {
        important_area(self, view_size, |_, si| Rect::from_size((0, 0), si))
    }

    fn take_focus(&mut self, _: cursive::direction::Direction) -> Result<EventResult, CannotFocus> {
        // this view is always focusable
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: cursive::event::Event) -> EventResult {
        on_event(
            self,
            event,
            |s, ev| match ev {
                Event::Key(Key::Right) if CONFIG.features.links => {
                    s.content.select_next_link();
                    s.check_and_update_viewport();
                    EventResult::consumed()
                }
                Event::Key(Key::Left) if CONFIG.features.links => {
                    s.content.select_prev_link();
                    s.check_and_update_viewport();
                    EventResult::consumed()
                }
                Event::Key(Key::Enter) if CONFIG.features.links => s.check_and_open_link(),
                Event::Mouse {
                    event: MouseEvent::Release(MouseButton::Left),
                    position,
                    offset,
                } => {
                    if let Some(element) = s.content.element_by_pos(position.saturating_sub(offset))
                    {
                        return match element.kind() {
                            ElementType::Link if CONFIG.features.links => {
                                s.content.select_link_by_id(element.id());
                                s.check_and_open_link()
                            }
                            _ => EventResult::Ignored,
                        };
                    }
                    EventResult::Ignored
                }
                _ => EventResult::Ignored,
            },
            |s| s.check_and_update_selection(),
            |s, si| s.important_area(si),
        )
    }
}

impl_scroller!(ArticleView::scroll);
