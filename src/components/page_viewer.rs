use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
};
use tokio::sync::mpsc::UnboundedSender;

use wiki_api::page::Page;

use crate::{
    action::{Action, ActionResult, PageViewerAction},
    config::Theme,
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

    theme: Theme,

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
        self.page.push(PageComponent::new(page, self.theme.clone()));

        if self.changing_page_language_popup.is_some() {
            self.changing_page_language_popup = None;
        }
    }

    fn pop(&mut self) {
        self.page.pop();
        self.page_n = self.page_n.saturating_sub(1);
    }

    pub fn get_page_language_selection_popup(&self) -> PageLanguageSelectionComponent {
        let language_links = self
            .current_page()
            .and_then(|x| x.page.language_links.to_owned())
            .unwrap_or_default();
        PageLanguageSelectionComponent::new(language_links, self.theme.clone())
    }
}

impl Component for PageViewer {
    fn init(&mut self, action_tx: UnboundedSender<Action>, theme: Theme) -> anyhow::Result<()> {
        self.action_tx = Some(action_tx);
        self.theme = theme;
        Ok(())
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if matches!(key.code, KeyCode::F(3)) {
            return Action::ShowPageLanguageSelection.into();
        }

        if matches!(key.code, KeyCode::Esc) {
            return Action::PageViewer(PageViewerAction::PopPage).into();
        }

        if let Some(page) = self.current_page_mut() {
            return page.handle_key_events(key);
        }

        ActionResult::Ignored
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
                self.theme.default_block().border_style(
                    Style::default()
                        .fg(self.theme.border_highlight_fg)
                        .bg(self.theme.border_highlight_bg),
                ),
                area,
            );
            f.render_widget(
                self.theme
                    .default_paragraph("Processing")
                    .alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if self.current_page().is_none() {
            f.render_widget(
                self.theme
                    .default_paragraph("No page opened")
                    .alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if let Some(page) = self.current_page_mut() {
            page.render(f, area);
        }
    }
}
