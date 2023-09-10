use std::collections::HashMap;

use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::Rect,
    text::{Line, Span},
    widgets::{Paragraph, ScrollbarState},
};
use tokio::sync::mpsc;
use tracing::{debug, error};
use wiki_api::{
    languages::Language,
    page::{Page, PageRequest},
    Endpoint,
};

use crate::{action::Action, components::Component, renderer::RenderedDocument, terminal::Frame};

#[cfg(debug_assertions)]
use crate::renderer::test_renderer::{render_nodes_raw, render_tree_data, render_tree_raw};

use super::root::Context;

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

#[derive(Default)]
pub struct PageComponent {
    page: Option<Page>,
    renderer: Renderer,
    render_cache: HashMap<u16, RenderedDocument>,

    endpoint: Option<Endpoint>,
    language: Option<Language>,

    scroll_state: ScrollbarState,
    scroll: usize,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl PageComponent {
    fn build_page(&self, title: String) -> Result<PageRequest> {
        let endpoint = self
            .endpoint
            .clone()
            .ok_or(anyhow!("No Endpoint configured"))?;
        let language = self
            .language
            .clone()
            .ok_or(anyhow!("No Language configured"))?;

        Ok(Page::builder()
            .page(title)
            .endpoint(endpoint)
            .language(language))
    }

    fn open_page(&mut self, title: String) {
        let tx = self.action_tx.clone().unwrap();
        let page_request = match self.build_page(title) {
            Ok(page_request) => page_request,
            Err(error) => {
                error!("Unable to build the page request: {:?}", error);
                return;
            }
        };
        tokio::spawn(async move {
            tx.send(Action::EnterProcessing).unwrap();
            match page_request.fetch().await {
                Ok(page) => tx.send(Action::FinishPage(page)).unwrap(),
                Err(error) => error!("Unable to complete the fetch: {:?}", error),
            };
            tx.send(Action::ExitProcessing).unwrap();
        });
    }

    fn render_page(&mut self, width: u16) -> &RenderedDocument {
        if self.render_cache.get(&width).is_some() {
            return self.render_cache.get(&width).unwrap();
        }

        let document = match self.renderer {
            Renderer::Default => RenderedDocument { lines: Vec::new() },
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeData => {
                render_tree_data(&self.page.as_ref().unwrap().content)
            }
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeRaw => render_tree_raw(&self.page.as_ref().unwrap().content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererNodeRaw => render_nodes_raw(&self.page.as_ref().unwrap().content),
        };

        self.render_cache.insert(width, document);
        self.render_cache.get(&width).unwrap()
    }

    fn switch_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;

        self.render_cache.clear();
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
        self.scroll_state = self.scroll_state.position(self.scroll as u16);
    }

    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
        self.scroll_state = self.scroll_state.position(self.scroll as u16);
    }
}

impl Component for PageComponent {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(sender);

        // FIXME: the endpoint and language should be set by the root component
        self.endpoint = Some(Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap());
        self.language = Some(Language::default());

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('s') => Action::EnterContext(Context::Search),
            KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
                Action::EnterContext(Context::Home)
            }
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
                Action::SwitchRenderer(self.renderer.next())
            }
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::OpenPage(title) => self.open_page(title),
            Action::FinishPage(page) => self.page = Some(page),
            Action::SwitchRenderer(renderer) => self.switch_renderer(renderer),
            Action::ScrollUp(amount) => self.scroll_up(amount),
            Action::ScrollDown(amount) => self.scroll_down(amount),
            _ => (),
        }
        None
    }

    fn render(&mut self, frame: &mut Frame, size: Rect) {
        if self.page.is_none() {
            frame.render_widget(Paragraph::new("Processing"), size);
            return;
        }

        let viewport_top = size.top().saturating_add(self.scroll as u16) as usize;
        let viewport_bottom = size.bottom().saturating_add(self.scroll as u16) as usize;

        let rendered_page = self.render_page(size.width);
        let lines: Vec<Line> = rendered_page
            .lines
            .iter()
            .enumerate()
            .filter(|(y, _)| &viewport_top <= y && y <= &viewport_bottom)
            .map(|(_, line)| {
                let mut spans: Vec<Span> = Vec::new();
                line.iter()
                    .map(|word| spans.push(Span::styled(word.content.to_string(), word.style)))
                    .count();
                Line {
                    spans,
                    ..Default::default()
                }
            })
            .collect();

        frame.render_widget(Paragraph::new(lines), size);
    }
}
