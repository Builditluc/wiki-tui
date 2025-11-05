use std::{collections::HashMap, sync::Arc};

use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use wiki_api::page::Page;

use crate::{
    action::{Action, ActionResult, PageViewerAction},
    config::{Config, Theme},
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
    page_cache: HashMap<Uuid, PageComponent>,

    is_processing: bool,
    changing_page_language_popup: Option<PageLanguageSelectionComponent>,

    config: Arc<Config>,
    theme: Arc<Theme>,

    action_tx: Option<UnboundedSender<Action>>,
}

impl PageViewer {
    fn cache_path() -> std::path::PathBuf {
        let mut path = directories::ProjectDirs::from("com", "github", "wiki-tui")
            .unwrap()
            .cache_dir()
            .to_path_buf();
        path.push("page_cache.json");
        path
    }

    fn load_cache(&mut self) {
        let path = Self::cache_path();
        if !path.exists() {
            return;
        }

        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return,
        };

        let reader = std::io::BufReader::new(file);
        let cache: HashMap<Uuid, PageComponent> = match serde_json::from_reader(reader) {
            Ok(cache) => cache,
            Err(_) => return,
        };

        self.page_cache = cache;
    }

    fn save_cache(&self) {
        let path = Self::cache_path();
        let file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(_) => return,
        };

        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self.page_cache).ok();
    }

    fn current_page_mut(&mut self) -> Option<&mut PageComponent> {
        self.page.get_mut(self.page_n)
    }

    pub fn current_page(&self) -> Option<&PageComponent> {
        self.page.get(self.page_n)
    }

    fn display_page(&mut self, page: Page) {
        self.page_n = self.page.len();
        if let Some(mut cached_page) = self.page_cache.get(&page.uuid).cloned() {
            cached_page.rebuild(self.config.clone(), self.theme.clone());
            self.page.push(cached_page);
        } else {
            let new_page = PageComponent::new(page, self.config.clone(), self.theme.clone());
            self.page_cache.insert(new_page.page.uuid, new_page.clone());
            self.page.push(new_page);
            self.save_cache();
        }

        if self.changing_page_language_popup.is_some() {
            self.changing_page_language_popup = None;
        }

        // always disable the processing screen when displaying a page
        self.is_processing = false;
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
        PageLanguageSelectionComponent::new(language_links, self.config.clone(), self.theme.clone())
    }
}

impl Component for PageViewer {
    fn init(
        &mut self,
        action_tx: UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(action_tx);
        self.config = config;
        self.theme = theme;
        self.load_cache();
        Ok(())
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self
            .config
            .bindings
            .page
            .toggle_page_language_selection
            .matches_event(key)
        {
            return Action::ShowPageLanguageSelection.into();
        }

        if self.config.bindings.page.pop_page.matches_event(key) {
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
                PageViewerAction::ExitLoading => self.is_processing = false,
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
