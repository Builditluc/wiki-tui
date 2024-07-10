use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use wiki_api::{
    languages::Language,
    page::{LanguageLink, Link, Page, Property},
    search::SearchResult,
    Endpoint,
};

use crate::action::{Action, PageViewerAction};

/// Responsible for loading a page
pub struct PageLoader {
    action_tx: UnboundedSender<Action>,
}

impl PageLoader {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        Self { action_tx }
    }

    pub fn load_search_result(&self, result: SearchResult) {
        self.load_page_custom(result.endpoint, result.language, result.title);
    }

    pub fn load_link(&self, link: Link) {
        let link_data = match link {
            Link::Internal(data) => data,
            _ => return,
        };

        self.load_page_custom(link_data.endpoint, link_data.language, link_data.page);
    }

    pub fn load_language_link(&self, link: LanguageLink) {
        self.load_page_custom(link.endpoint, link.language, link.title);
    }

    fn load_page_custom(&self, endpoint: Endpoint, language: Language, title: String) {
        let page_request = Page::builder()
            .page(title)
            .properties(vec![
                Property::Text,
                Property::Sections,
                Property::LangLinks,
            ])
            .endpoint(endpoint)
            .language(language);

        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            tx.send(Action::SwitchContextPage).unwrap();
            tx.send(Action::EnterProcessing).unwrap();

            match page_request.fetch().await {
                Ok(page) => tx
                    .send(Action::PageViewer(PageViewerAction::DisplayPage(page)))
                    .unwrap(),
                Err(error) => error!("Unable to fetch the page: {:?}", error),
            };

            tx.send(Action::EnterNormal).unwrap();
        });
    }
}
