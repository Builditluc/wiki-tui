use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use wiki_api::page::Page;

use crate::{
    action::{Action, ActionResult, PageViewerAction},
    key_event,
    terminal::Frame,
    ui::centered_rect,
};

use super::{page::PageComponent, page_language_popup::PageLanguageSelectionComponent, Component};

/// Can display multiple pages and supports selecting between them
/// Responsible for fetching the pages and managing them (NOT rendering)
#[derive(Default)]
pub struct PageViewer {
    page: Vec<PageComponent>,
    page_n: usize,

    is_processing: bool,
    changing_page_language_popup: Option<PageLanguageSelectionComponent>,

    action_tx: Option<UnboundedSender<Action>>,
}

impl PageViewer {
    fn current_page_mut(&mut self) -> Option<&mut PageComponent> {
        self.page.get_mut(self.page_n)
    }

    fn current_page(&self) -> Option<&PageComponent> {
        self.page.get(self.page_n)
    }

    fn display_page(&mut self, page: Page) {
        self.page_n = self.page.len();
        self.page.push(PageComponent::new(page));

        if self.changing_page_language_popup.is_some() {
            self.changing_page_language_popup = None;
        }
    }

    fn pop(&mut self) {
        self.page.pop();
        self.page_n = self.page_n.saturating_sub(1);
    }
}

impl Component for PageViewer {
    fn init(&mut self, action_tx: UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(action_tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self.changing_page_language_popup.is_some() {
            if matches!(key.code, KeyCode::F(3) | KeyCode::Esc) {
                self.changing_page_language_popup = None;
                return ActionResult::consumed();
            }

            return self
                .changing_page_language_popup
                .as_mut()
                .unwrap()
                .handle_key_events(key);
        }

        if matches!(key.code, KeyCode::F(3)) {
            let language_links = self
                .current_page()
                .and_then(|x| x.page.language_links.to_owned())
                .unwrap_or_default();
            self.changing_page_language_popup =
                Some(PageLanguageSelectionComponent::new(language_links));
            return ActionResult::consumed();
        }

        if matches!(key.code, KeyCode::Esc) {
            return Action::PageViewer(PageViewerAction::PopPage).into();
        }

        if let Some(page) = self.current_page_mut() {
            return page.handle_key_events(key);
        }

        ActionResult::Ignored
    }

    fn keymap(&self) -> super::help::Keymap {
        let mut keymap = vec![(
            key_event!(Key::Esc),
            Action::PageViewer(PageViewerAction::PopPage).into(),
        )];

        if let Some(page) = self.current_page() {
            keymap.append(&mut page.keymap());
        }

        keymap
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::PageViewer(page_viewer_action) => match page_viewer_action {
                PageViewerAction::DisplayPage(page) => self.display_page(page),
                PageViewerAction::PopPage => self.pop(),
            },
            Action::EnterProcessing => self.is_processing = true,
            Action::EnterNormal => self.is_processing = false,
            _ => {
                if let Some(ref mut popup) = self.changing_page_language_popup {
                    return popup.update(action);
                }
                if let Some(page) = self.current_page_mut() {
                    return page.update(action);
                }
                return ActionResult::Ignored;
            }
        }
        ActionResult::consumed()
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        if self.is_processing {
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)),
                area,
            );
            f.render_widget(
                Paragraph::new("Processing").alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if self.current_page().is_none() {
            f.render_widget(
                Paragraph::new("No page opened").alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if let Some(page) = self.current_page_mut() {
            page.render(f, area);
        }

        if let Some(ref mut page_language_popup) = self.changing_page_language_popup {
            page_language_popup.render(f, area);
        }
    }
}
