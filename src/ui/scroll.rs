use cursive::{
    event::{Event, EventResult, Key, MouseButton, MouseEvent},
    Rect, Vec2,
};

const SCROLL_STRATEGY: ScrollStrategy = ScrollStrategy::KeepRow;
const SCROLL_WHEEL_DOWN: usize = 3;
const SCROLL_WHEEL_UP: usize = 3;
const SCROLL_PAGE_UP: usize = 10;
const SCROLL_PAGE_DOWN: usize = 10;

pub use cursive::view::scroll::{draw, layout, required_size, Core, ScrollStrategy, Scroller};

use crate::config::CONFIG;

/// Implements `View::important_area` on the given model
pub fn important_area<T, ImportantArea>(
    scroller: &T,
    size: Vec2,
    mut important_area: ImportantArea,
) -> Rect
where
    T: Scroller,
    ImportantArea: FnMut(&T, Vec2) -> Rect,
{
    let viewport = scroller.get_scroller().content_viewport();
    let area = important_area(scroller, size);
    let top_left = area.top_left().saturating_sub(viewport.top_left());
    let bot_right = area
        .bottom_right()
        .saturating_sub(viewport.top_left())
        .or_min(viewport.bottom_right());
    Rect::from_corners(top_left, bot_right)
}

/// Implements `View::on_event` on the given model
pub fn on_event<T, OnEvent, OnScroll, ImportantArea>(
    scroller: &mut T,
    event: Event,
    mut on_event: OnEvent,
    mut on_scroll: OnScroll,
    mut important_area: ImportantArea,
) -> EventResult
where
    T: Scroller,
    OnEvent: FnMut(&mut T, Event) -> EventResult,
    OnScroll: FnMut(&mut T),
    ImportantArea: FnMut(&T, Vec2) -> Rect,
{
    let mut relative_event = event.clone();
    let inside = scroller
        .get_scroller_mut()
        .is_event_inside(&mut relative_event);
    let result = if inside {
        on_event(scroller, relative_event)
    } else {
        EventResult::Ignored
    };

    let half_viewport_height = scroller.get_scroller().content_viewport().height() / 2;

    match result {
        EventResult::Ignored => {
            match event {
                Event::Key(Key::Home) if scroller.get_scroller_mut().is_enabled().any() => {
                    scroller.get_scroller_mut().scroll_to_left();
                    scroller.get_scroller_mut().scroll_to_top();
                }
                Event::Key(Key::End) if scroller.get_scroller_mut().is_enabled().any() => {
                    scroller.get_scroller_mut().scroll_to_right();
                    scroller.get_scroller_mut().scroll_to_bottom();
                }
                Event::Key(Key::PageUp) if scroller.get_scroller_mut().can_scroll_up() => {
                    scroller.get_scroller_mut().scroll_up(SCROLL_PAGE_UP)
                }
                Event::Key(Key::PageDown) if scroller.get_scroller_mut().can_scroll_down() => {
                    scroller.get_scroller_mut().scroll_down(SCROLL_PAGE_DOWN)
                }
                Event::Char('G') => scroller.get_scroller_mut().scroll_to_bottom(),
                // TODO(enoumy): Make this be gg in short sequence.
                Event::Char('g') => scroller.get_scroller_mut().scroll_to_top(),

                Event::CtrlChar('d') => {
                    if scroller.get_scroller_mut().can_scroll_down() {
                        scroller
                            .get_scroller_mut()
                            .scroll_down(half_viewport_height)
                    }
                }
                Event::CtrlChar('u') => {
                    if scroller.get_scroller_mut().can_scroll_up() {
                        scroller.get_scroller_mut().scroll_up(half_viewport_height)
                    }
                }
                key if key == CONFIG.keybindings.down
                    && scroller.get_scroller_mut().can_scroll_down() =>
                {
                    scroller.get_scroller_mut().scroll_down(1)
                }
                key if key == CONFIG.keybindings.up
                    && scroller.get_scroller_mut().can_scroll_up() =>
                {
                    scroller.get_scroller_mut().scroll_up(1)
                }
                key if key == CONFIG.keybindings.left
                    && scroller.get_scroller_mut().can_scroll_left() =>
                {
                    scroller.get_scroller_mut().scroll_left(1);
                }
                key if key == CONFIG.keybindings.right
                    && scroller.get_scroller_mut().can_scroll_right() =>
                {
                    scroller.get_scroller_mut().scroll_right(1);
                }
                Event::Mouse {
                    event: MouseEvent::WheelUp,
                    ..
                } if scroller.get_scroller_mut().can_scroll_up() => {
                    scroller.get_scroller_mut().scroll_up(SCROLL_WHEEL_UP);
                }
                Event::Mouse {
                    event: MouseEvent::WheelDown,
                    ..
                } if scroller.get_scroller_mut().can_scroll_down() => {
                    scroller.get_scroller_mut().scroll_down(SCROLL_WHEEL_DOWN);
                }
                Event::Mouse {
                    event: MouseEvent::Press(MouseButton::Left),
                    position,
                    offset,
                } if scroller.get_scroller_mut().get_show_scrollbars()
                    && position
                        .checked_sub(offset)
                        .map(|position| scroller.get_scroller_mut().start_drag(position))
                        .unwrap_or(false) =>
                {
                    return EventResult::consumed();
                }
                Event::Mouse {
                    event: MouseEvent::Hold(MouseButton::Left),
                    position,
                    offset,
                } if scroller.get_scroller_mut().get_show_scrollbars() => {
                    let position = position.saturating_sub(offset);
                    scroller.get_scroller_mut().drag(position);
                }
                Event::Mouse {
                    event: MouseEvent::Release(MouseButton::Left),
                    ..
                } => scroller.get_scroller_mut().release_grab(),
                _ => return EventResult::Ignored,
            }
            scroller
                .get_scroller_mut()
                .set_scroll_strategy(SCROLL_STRATEGY);
            on_scroll(scroller);
            EventResult::consumed()
        }
        other => {
            let inner_size = scroller.get_scroller_mut().inner_size();
            let important = important_area(scroller, inner_size);
            scroller.get_scroller_mut().scroll_to_rect(important);

            other
        }
    }
}
