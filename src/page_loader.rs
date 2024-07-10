use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use wiki_api::{
    languages::Language,
    page::{LanguageLink, Link, Page, Property},
    Endpoint,
};

use crate::action::{Action, PageViewerAction};

/// Responsible for loading a page
pub struct PageLoader {
    endpoint: Endpoint,
    language: Language,

    action_tx: UnboundedSender<Action>,
}

impl PageLoader {
    pub fn new(endpoint: Endpoint, language: Language, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            endpoint,
            language,
            action_tx,
        }
    }

    pub fn load_page(&self, title: String) {
        self.load_page_custom(self.endpoint.clone(), self.language.clone(), title);
    }

    pub fn load_link(&self, link: Link) {
        let link_data = match link {
            Link::Internal(data) => data,
            _ => return,
        };

        self.load_page_custom(link_data.endpoint, link_data.language, link_data.page);
    }

    pub fn load_language_link(&self, link: LanguageLink) {
        let mut endpoint = self.endpoint.clone();
        let _ = endpoint.set_host(Some(link.url.host_str().unwrap()));
        self.load_page_custom(endpoint, link.language, link.title);
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
