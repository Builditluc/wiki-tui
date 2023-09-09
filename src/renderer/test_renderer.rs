use ratatui::{style::Style, widgets::LineGauge};
use wiki_api::document::Document;

use super::{Line, RenderedDocument, Renderer, Word};

pub struct TestRenderer;

impl Renderer for TestRenderer {
    fn render<'a>(&self, document: &'a Document, width: u16) -> RenderedDocument<'a> {
        let mut lines: Vec<Line> = Vec::new();

        for ref raw in document.nodes.iter() {
            lines.push(vec![Word {
                node: document.nth(raw.index).unwrap(),
                content: format!("{:?}", raw.data),
                style: Style::default(),
                width: 0.0,
                whitespace_width: 0.0,
                penalty_width: 0.0,
            }])
        }

        RenderedDocument { lines }
    }
}
