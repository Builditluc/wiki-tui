use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;
use wiki_api::page::Page;

use crate::{
    action::{Action, PageViewerAction},
    terminal::Frame,
    ui::centered_rect,
};

use super::{page::PageComponent, Component};

/// Can display multiple pages and supports selecting between them
/// Responsible for fetching the pages and managing them (NOT rendering)
#[derive(Default)]
pub struct PageViewer {
    page: Vec<PageComponent>,
    page_n: usize,

    is_processing: bool,

    action_tx: Option<UnboundedSender<Action>>,
}

impl PageViewer {
    fn current_page_mut(&mut self) -> Option<&mut PageComponent> {
        self.page.get_mut(self.page_n)
    }

    fn display_page(&mut self, page: Page) {
        self.page_n = self.page.len();
        self.page.push(PageComponent::new(page));
    }
}

impl Component for PageViewer {
    fn init(&mut self, action_tx: UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(action_tx);
        Ok(())
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::PageViewer(page_viewer_action) => match page_viewer_action {
                PageViewerAction::DisplayPage(page) => self.display_page(page),
            },
            Action::EnterProcessing => self.is_processing = true,
            Action::ExitProcessing => self.is_processing = false,
            _ => {
                if let Some(page) = self.current_page_mut().as_mut() {
                    return page.dispatch(action);
                }
            }
        }

        None
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
        if let Some(page) = self.current_page_mut() {
            page.render(f, area);
            return;
        }
        f.render_widget(
            Paragraph::new("No page opened").alignment(Alignment::Center),
            centered_rect(area, 100, 50),
        );
    }
}
