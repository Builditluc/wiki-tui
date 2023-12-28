use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Margin, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use tracing::{debug, info};
use wiki_api::{document::Data, page::Page};

use crate::{
    action::{Action, ActionResult, PageAction},
    components::Component,
    has_modifier, key_event,
    renderer::{default_renderer::render_document, RenderedDocument},
    terminal::Frame,
    ui::padded_rect,
};

#[cfg(debug_assertions)]
use crate::renderer::test_renderer::{render_nodes_raw, render_tree_data, render_tree_raw};

const SCROLLBAR: bool = true;
const LINK_SELECT: bool = true;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Renderer {
    #[default]
    Default,

    #[cfg(debug_assertions)]
    TestRendererTreeData,
    #[cfg(debug_assertions)]
    TestRendererTreeRaw,
    #[cfg(debug_assertions)]
    TestRendererNodeRaw,
}

impl Renderer {
    pub fn next(&self) -> Self {
        match self {
            #[cfg(not(debug_assertions))]
            &Renderer::Default => Renderer::Default,

            #[cfg(debug_assertions)]
            &Renderer::Default => Renderer::TestRendererTreeData,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererTreeData => Renderer::TestRendererTreeRaw,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererTreeRaw => Renderer::TestRendererNodeRaw,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererNodeRaw => Renderer::Default,
        }
    }
}

pub struct PageComponent {
    page: Page,
    renderer: Renderer,
    render_cache: HashMap<u16, RenderedDocument>,
    viewport: Rect,
    selected: (usize, usize),
}

impl PageComponent {
    pub fn new(page: Page) -> Self {
        Self {
            page,
            renderer: Renderer::default(),
            render_cache: HashMap::new(),
            viewport: Rect::default(),
            selected: (0, 0),
        }
    }

    fn render_page(&self, width: u16) -> RenderedDocument {
        match self.renderer {
            Renderer::Default => render_document(&self.page.content, width),
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeData => render_tree_data(&self.page.content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeRaw => render_tree_raw(&self.page.content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererNodeRaw => render_nodes_raw(&self.page.content),
        }
    }

    fn switch_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
        self.flush_cache();
    }

    fn flush_cache(&mut self) {
        debug!("flushing '{}' cached renders", self.render_cache.len());
        self.render_cache.clear();
        if LINK_SELECT {
            self.selected = (0, 0);
        }
    }

    fn scroll_down(&mut self, amount: u16) {
        self.viewport.y += amount;
    }

    fn scroll_up(&mut self, amount: u16) {
        self.viewport.y = self.viewport.y.saturating_sub(amount);
    }

    fn select_first(&mut self) {
        if self.page.content.nth(0).is_none() {
            return;
        }

        let selectable_node = self
            .page
            .content
            .nth(0)
            .unwrap()
            .descendants()
            .find(|node| matches!(node.data(), &Data::WikiLink { .. }));

        if let Some(selectable_node) = selectable_node {
            let first_index = selectable_node.index();
            let last_index = selectable_node
                .last_child()
                .map(|child| child.index())
                .unwrap_or(first_index);
            self.selected = (first_index, last_index);
        }
    }

    fn select_prev(&mut self) {
        if self.page.content.nth(0).is_none() {
            return;
        }

        let selectable_node = self
            .page
            .content
            .nth(0)
            .unwrap()
            .descendants()
            .filter(|node| {
                matches!(node.data(), &Data::WikiLink { .. }) && node.index() < self.selected.0
            })
            .last();

        if let Some(selectable_node) = selectable_node {
            let first_index = selectable_node.index();
            let last_index = selectable_node
                .last_child()
                .map(|child| child.index())
                .unwrap_or(first_index);
            self.selected = (first_index, last_index);
        }
    }

    fn select_next(&mut self) {
        if self.page.content.nth(0).is_none() {
            return;
        }

        let selectable_node = self
            .page
            .content
            .nth(0)
            .unwrap()
            .descendants()
            .find(|node| {
                matches!(node.data(), &Data::WikiLink { .. }) && self.selected.1 < node.index()
            });

        if let Some(selectable_node) = selectable_node {
            let first_index = selectable_node.index();
            let last_index = selectable_node
                .last_child()
                .map(|child| child.index())
                .unwrap_or(first_index);
            self.selected = (first_index, last_index);
        }
    }

    fn select_last(&mut self) {
        if self.page.content.nth(0).is_none() {
            return;
        }

        let selectable_node = self
            .page
            .content
            .nth(0)
            .unwrap()
            .descendants()
            .filter(|node| {
                matches!(node.data(), &Data::WikiLink { .. }) && node.index() > self.selected.1
            })
            .last();

        if let Some(selectable_node) = selectable_node {
            let first_index = selectable_node.index();
            let last_index = selectable_node
                .last_child()
                .map(|child| child.index())
                .unwrap_or(first_index);
            self.selected = (first_index, last_index);
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        self.viewport.height = height;

        self.flush_cache();
    }
}

impl Component for PageComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match key.code {
            KeyCode::Char('r') if has_modifier!(key, Modifier::CONTROL) => {
                Action::Page(PageAction::SwitchRenderer(self.renderer.next())).into()
            }
            KeyCode::Left if has_modifier!(key, Modifier::SHIFT) => {
                Action::Page(PageAction::SelectFirstLink).into()
            }
            KeyCode::Right if has_modifier!(key, Modifier::SHIFT) => {
                Action::Page(PageAction::SelectLastLink).into()
            }
            KeyCode::Up if has_modifier!(key, Modifier::SHIFT) => {
                Action::Page(PageAction::SelectTopLink).into()
            }
            KeyCode::Down if has_modifier!(key, Modifier::SHIFT) => {
                Action::Page(PageAction::SelectBottomLink).into()
            }
            KeyCode::Left => Action::Page(PageAction::SelectPrevLink).into(),
            KeyCode::Right => Action::Page(PageAction::SelectNextLink).into(),
            _ => ActionResult::Ignored,
        }
    }

    fn keymap(&self) -> super::help::Keymap {
        vec![
            (
                key_event!('r', Modifier::CONTROL),
                Action::Page(PageAction::SwitchRenderer(self.renderer.next())).into(),
            ),
            (
                key_event!(Key::Left, Modifier::SHIFT),
                Action::Page(PageAction::SelectFirstLink).into(),
            ),
            (
                key_event!(Key::Left),
                Action::Page(PageAction::SelectPrevLink).into(),
            ),
            (
                key_event!(Key::Right, Modifier::SHIFT),
                Action::Page(PageAction::SelectLastLink).into(),
            ),
            (
                key_event!(Key::Right),
                Action::Page(PageAction::SelectNextLink).into(),
            ),
            (
                key_event!(Key::Up, Modifier::SHIFT),
                Action::Page(PageAction::SelectTopLink).into(),
            ),
            (
                key_event!(Key::Down, Modifier::SHIFT),
                Action::Page(PageAction::SelectBottomLink).into(),
            ),
        ]
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Page(page_action) => match page_action {
                PageAction::SwitchRenderer(renderer) => self.switch_renderer(renderer),

                PageAction::SelectFirstLink => self.select_first(),
                PageAction::SelectLastLink => self.select_last(),

                PageAction::SelectTopLink | PageAction::SelectBottomLink => todo!(),

                PageAction::SelectPrevLink => self.select_prev(),
                PageAction::SelectNextLink => self.select_next(),
            },
            Action::ScrollUp(amount) => self.scroll_up(amount),
            Action::ScrollDown(amount) => self.scroll_down(amount),
            Action::Resize(width, heigth) => self.resize(width, heigth),
            _ => return ActionResult::Ignored,
        }
        ActionResult::consumed()
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let area = padded_rect(area, 1, 1);
        let page_area = if SCROLLBAR {
            area.inner(&Margin {
                vertical: 0,
                horizontal: 2, // for the scrollbar
            })
        } else {
            area
        };

        self.viewport.width = page_area.width;
        self.viewport.height = page_area.height;

        let rendered_page = match self.render_cache.get(&page_area.width) {
            Some(rendered_page) => rendered_page,
            None => {
                let rendered_page = self.render_page(page_area.width);
                info!("rebuilding cache for '{}'", page_area.width);
                self.render_cache.insert(page_area.width, rendered_page);
                self.render_cache.get(&page_area.width).unwrap()
            }
        };

        let lines: Vec<Line> = rendered_page
            .lines
            .iter()
            .skip(self.viewport.top() as usize)
            .take(self.viewport.bottom() as usize)
            .map(|line| {
                let mut spans: Vec<Span> = Vec::new();
                line.iter()
                    .map(|word| {
                        let mut span = Span::styled(
                            format!(
                                "{}{}",
                                word.content,
                                " ".repeat(word.whitespace_width as usize)
                            ),
                            word.style,
                        );

                        if let Some(node) = word.node(&self.page.content) {
                            let index = node.index();
                            if self.selected.0 <= index && index <= self.selected.1 {
                                span.patch_style(Style::new().add_modifier(Modifier::UNDERLINED))
                            }
                        }

                        spans.push(span);
                    })
                    .count();
                Line {
                    spans,
                    ..Default::default()
                }
            })
            .collect();

        if SCROLLBAR {
            let scrollbar = Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some(" "))
                .track_style(Style::new().black().on_black())
                .thumb_style(Style::new().blue())
                .orientation(ScrollbarOrientation::VerticalRight);
            let mut scrollbar_state = ScrollbarState::new(rendered_page.lines.len())
                .position(self.viewport.top() as usize);
            f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }

        f.render_widget(Paragraph::new(lines), page_area);
    }
}
