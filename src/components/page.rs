use anyhow::{anyhow, Result};
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    style::StyledContent,
};
use ratatui::{
    prelude::Rect,
    text::{Line, Span, Spans, StyledGrapheme},
    widgets::Paragraph,
};
use tokio::sync::mpsc;
use tracing::{debug, error};
use wiki_api::{
    languages::Language,
    page::{Page, PageRequest},
    Endpoint,
};

use crate::{
    action::Action,
    components::Component,
    renderer::{test_renderer::TestRenderer, RenderedDocument, Renderer},
    terminal::Frame,
};

use super::root::Context;

#[derive(Default)]
pub struct PageComponent {
    page: Option<Page>,
    renderer: Option<Box<dyn Renderer + Send>>,

    endpoint: Option<Endpoint>,
    language: Option<Language>,

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

    fn render_page(&self, width: u16) -> RenderedDocument {
        self.renderer
            .as_ref()
            .unwrap()
            .render(&self.page.as_ref().unwrap().content, width)
    }
}

impl Component for PageComponent {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(sender);

        // FIXME: the endpoint and language should be set by the root component
        self.endpoint = Some(Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap());
        self.language = Some(Language::default());

        self.renderer = Some(Box::new(TestRenderer));

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('s') => Action::EnterContext(Context::Search),
            KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
                Action::EnterContext(Context::Home)
            }
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::OpenPage(title) => self.open_page(title),
            Action::FinishPage(page) => self.page = Some(page),
            _ => (),
        }
        None
    }

    fn render(&mut self, frame: &mut Frame, size: Rect) {
        if self.page.is_none() {
            frame.render_widget(Paragraph::new("Processing"), size);
            return;
        }

        let rendered_page = self.render_page(size.width);
        let lines: Vec<Line> = rendered_page
            .lines
            .iter()
            .map(|line| {
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
