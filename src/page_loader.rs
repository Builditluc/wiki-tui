use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use wiki_api::{
    languages::Language,
    page::{Page, Property},
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
        let page_request = Page::builder()
            .page(title)
            .properties(vec![
                Property::Text,
                Property::Sections,
                Property::LangLinks,
            ])
            .endpoint(self.endpoint.clone())
            .language(self.language.clone());

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
