use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub fn padded_rect(r: Rect, pad_y: u16, pad_x: u16) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(pad_x),
            Constraint::Length(r.width.saturating_sub(pad_x * 2)),
            Constraint::Length(pad_x),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(pad_y),
            Constraint::Length(chunks[1].width.saturating_sub(pad_y * 2)),
            Constraint::Length(pad_y),
        ])
        .split(chunks[1])[1]
}
