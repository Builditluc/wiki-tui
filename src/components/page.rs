use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Margin, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use tracing::debug;
use wiki_api::page::Page;

use crate::{
    action::{Action, PageAction},
    components::Component,
    renderer::{default_renderer::render_document, RenderedDocument},
    terminal::Frame,
    ui::padded_rect,
};

#[cfg(debug_assertions)]
use crate::renderer::test_renderer::{render_nodes_raw, render_tree_data, render_tree_raw};

const SCROLLBAR: bool = true;

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
    scroll: usize,
}

impl PageComponent {
    pub fn new(page: Page) -> Self {
        Self {
            page,
            renderer: Renderer::default(),
            render_cache: HashMap::new(),
            scroll: 0,
        }
    }

    fn render_page(&mut self, width: u16) -> &RenderedDocument {
        if self.render_cache.get(&width).is_some() {
            return self.render_cache.get(&width).unwrap();
        }

        debug!(
            "rebuilding cache for renderer '{:?}' with width '{}'",
            self.renderer, width
        );
        let document = match self.renderer {
            Renderer::Default => render_document(&self.page.content, width),
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeData => render_tree_data(&self.page.content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeRaw => render_tree_raw(&self.page.content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererNodeRaw => render_nodes_raw(&self.page.content),
        };

        self.render_cache.insert(width, document);
        self.render_cache.get(&width).unwrap()
    }

    fn switch_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
        self.flush_cache();
    }

    fn flush_cache(&mut self) {
        debug!("flushing '{}' cached renders", self.render_cache.len());
        self.render_cache.clear();
        self.scroll = 0;
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
    }

    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }
}

impl Component for PageComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
                Action::Page(PageAction::SwitchRenderer(self.renderer.next()))
            }
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Page(page_action) => match page_action {
                PageAction::SwitchRenderer(renderer) => self.switch_renderer(renderer),
            },
            Action::ScrollUp(amount) => self.scroll_up(amount),
            Action::ScrollDown(amount) => self.scroll_down(amount),
            Action::Resize(..) => self.flush_cache(),
            _ => (),
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let area = padded_rect(area, 1, 1);
        let page_area = if SCROLLBAR {
            area.inner(&Margin {
                vertical: 0,
                horizontal: 1, // for the scrollbar
            })
        } else {
            area
        };

        let viewport_top = self.scroll;
        let viewport_bottom = viewport_top.saturating_add(page_area.height as usize);

        let rendered_page = self.render_page(page_area.width);
        let lines: Vec<Line> = rendered_page
            .lines
            .iter()
            .enumerate()
            .skip(viewport_top)
            .take(viewport_bottom)
            .map(|(_, line)| {
                let mut spans: Vec<Span> = Vec::new();
                line.iter()
                    .map(|word| {
                        spans.push(Span::styled(
                            format!(
                                "{}{}",
                                word.content,
                                " ".repeat(word.whitespace_width as usize)
                            ),
                            word.style,
                        ));
                    })
                    .count();
                Line {
                    spans,
                    ..Default::default()
                }
            })
            .collect();

        if SCROLLBAR {
            let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);
            let mut scrollbar_state =
                ScrollbarState::new(rendered_page.lines.len()).position(self.scroll);
            f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }

        f.render_widget(Paragraph::new(lines), page_area);
    }
}
