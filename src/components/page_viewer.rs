use std::{collections::HashMap, sync::Arc};

use ratatui::{
    prelude::{Alignment, Rect},
    style::Style,
};
use tracing::{debug, error};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::{
    action::{Action, ActionResult, PageViewerAction},
    config::{Config, Theme},
    terminal::Frame,
    ui::centered_rect,
};

use super::{page::PageComponent, page_language_popup::PageLanguageSelectionComponent, Component};

use wiki_api::{languages::Language, page::{Page, Property}};

/// Can display multiple pages and supports selecting between them
/// Responsible for fetching the pages and managing them (NOT rendering)
#[derive(Default)]
pub struct PageViewer {
    page: Vec<PageComponent>,
    page_n: usize,
    page_cache: HashMap<Uuid, PageComponent>,
    /// Maps (title, language_code) -> UUID for quick cache lookups
    page_identifier_index: HashMap<(String, String), Uuid>,

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
            debug!("no cache file found at {:?}", path);
            return;
        }

        let file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("failed to open cache file at {:?}: {}", path, e);
                return;
            }
        };

        let reader = std::io::BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(cache) => {
                debug!("successfully loaded cache from {:?}", path);
                self.page_cache = cache;
                self.rebuild_identifier_index();
            }
            Err(e) => {
                error!("failed to deserialize cache from {:?}: {}", path, e);
            }
        };
    }

    fn rebuild_identifier_index(&mut self) {
        self.page_identifier_index.clear();
        for (uuid, page_component) in &self.page_cache {
            let key = (
                page_component.page.title.clone(),
                page_component.page.language.code().to_string(),
            );
            self.page_identifier_index.insert(key, *uuid);
        }
        debug!("rebuilt identifier index with {} entries", self.page_identifier_index.len());
    }

    fn save_cache(&self) {
        let path = Self::cache_path();
        let file = match std::fs::File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("failed to create cache file at {:?}: {}", path, e);
                return;
            }
        };

        let writer = std::io::BufWriter::new(file);
        match serde_json::to_writer(writer, &self.page_cache) {
            Ok(_) => debug!("successfully saved cache to {:?}", path),
            Err(e) => error!("failed to serialize and save cache to {:?}: {}", path, e),
        }
    }

    /// Syncs all currently active pages back to the page_cache, then saves to disk
    pub fn sync_and_save_cache(&mut self) {
        debug!("syncing {} active pages to cache", self.page.len());
        for page_component in &self.page {
            let key = (
                page_component.page.title.clone(),
                page_component.page.language.code().to_string(),
            );
            self.page_cache.insert(page_component.page.uuid, page_component.clone());
            self.page_identifier_index.insert(key, page_component.page.uuid);
        }
        self.save_cache();
    }

    /// Check if a page is already cached by its identifier
    pub fn get_cached_page(&self, title: &str, language: Language) -> Option<Page> {
        let key = (title.to_string(), language.code().to_string());
        debug!("cache lookup for: title='{}', language='{}'", title, language.code());
        let uuid = self.page_identifier_index.get(&key)?;
        debug!("found uuid in index: {}", uuid);
        let page_component = self.page_cache.get(uuid)?;
        debug!("found page component in cache");
        Some(page_component.page.clone())
    }

    fn current_page_mut(&mut self) -> Option<&mut PageComponent> {
        self.page.get_mut(self.page_n)
    }

    pub fn current_page(&self) -> Option<&PageComponent> {
        self.page.get(self.page_n)
    }

    fn display_page(&mut self, page: Page) {
        self.page_n = self.page.len();
        debug!("display_page called for '{}' with uuid {}", page.title, page.uuid);

        // First try to find by UUID (exact match)
        if let Some(mut cached_page) = self.page_cache.get(&page.uuid).cloned() {
            debug!("found page in cache by uuid, using cached version");
            cached_page.rebuild(self.config.clone(), self.theme.clone());
            self.page.push(cached_page);
        } else {
            // UUID not found, check if we have this page by (title, language)
            let key = (page.title.clone(), page.language.code().to_string());

            if let Some(&existing_uuid) = self.page_identifier_index.get(&key) {
                // We have this page cached, but with a different UUID
                debug!("found existing page in index with different uuid {}, updating uuid to {}", existing_uuid, page.uuid);

                // Remove the old UUID entry and add with new UUID
                if let Some(mut existing_page) = self.page_cache.remove(&existing_uuid) {
                    // Update the page data with the new fetch (in case content changed)
                    existing_page.page = page.clone();
                    existing_page.rebuild(self.config.clone(), self.theme.clone());

                    // Store with new UUID and update index
                    self.page_cache.insert(page.uuid, existing_page.clone());
                    self.page_identifier_index.insert(key, page.uuid);
                    self.page.push(existing_page);
                } else {
                    // Index pointed to non-existent UUID, treat as new page
                    debug!("index pointed to non-existent uuid, creating new page");
                    let new_page = PageComponent::new(page.clone(), self.config.clone(), self.theme.clone());
                    self.page_cache.insert(page.uuid, new_page.clone());
                    self.page_identifier_index.insert(key, page.uuid);
                    self.page.push(new_page);
                }
            } else {
                // Truly new page, not in index at all
                debug!("page not in cache or index, creating new PageComponent");
                let new_page = PageComponent::new(page.clone(), self.config.clone(), self.theme.clone());
                debug!("adding page to cache and index with key: {:?}", key);
                self.page_cache.insert(new_page.page.uuid, new_page.clone());
                self.page_identifier_index.insert(key, new_page.page.uuid);
                self.page.push(new_page);
            }
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
            Action::TryLoadPage(title, language, endpoint) => {
                if let Some(cached_page) = self.get_cached_page(&title, language) {
                    debug!("cache hit for page '{}' - loading instantly", title);
                    self.action_tx.as_ref().unwrap()
                        .send(Action::SwitchContextPage).unwrap();
                    self.display_page(cached_page);
                    return ActionResult::consumed();
                } else {
                    debug!("cache miss for page '{}' - fetching from API", title);
                    // Cache miss - fetch from API
                    let page_request = wiki_api::page::Page::builder()
                        .page(title.clone())
                        .properties(vec![
                            Property::Text,
                            Property::Sections,
                            Property::LangLinks,
                        ])
                        .endpoint(endpoint)
                        .language(language)
                        .redirects(self.config.api.page_redirects);

                    let tx = self.action_tx.clone().unwrap();
                    tokio::spawn(async move {
                        tx.send(Action::SwitchContextPage).unwrap();
                        tx.send(Action::EnterProcessing).unwrap();

                        match page_request.fetch().await {
                            Ok(page) => tx
                                .send(Action::PageViewer(crate::action::PageViewerAction::DisplayPage(page)))
                                .unwrap(),
                            Err(error) => {
                                let error_msg = format!("Unable to fetch the page '{}': {}", title, error);
                                tracing::error!("{}", error_msg);
                                tx.send(Action::PageViewer(crate::action::PageViewerAction::ExitLoading))
                                    .unwrap();
                                tx.send(Action::PopupError(error_msg)).unwrap();
                            }
                        };

                        tx.send(Action::EnterNormal).unwrap();
                    });
                    return ActionResult::consumed();
                }
            }
            Action::PageViewer(page_viewer_action) => match page_viewer_action {
                PageViewerAction::DisplayPage(page) => self.display_page(page),
                PageViewerAction::PopPage => self.pop(),
                PageViewerAction::ExitLoading => self.is_processing = false,
                PageViewerAction::SaveCache => self.sync_and_save_cache(),
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
