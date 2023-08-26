use crate::{
    config,
    wiki::parser::{Data, Page},
};
use cursive::theme::{Color, Style};

const DISAMBIGUATION_PADDING: u8 = 1;
const DISAMBIGUATION_PREFIX: char = '|';

const LIST_ITEM_PADDING: u8 = 2;

const DESCRIPTION_LIST_TERM_PADDING: u8 = 2;
const DESCRIPTION_LIST_DESCRIPTION_PADDING: u8 = 4;

/// An element only containing the neccessary information for rendering (and an id so that it can
/// be referenced to an article element
#[derive(Debug)]
pub struct RenderedElement {
    /// The id of the ArticleElement this element belongs to
    pub id: usize,
    /// The content of the element
    pub content: String,
    /// The style of the element
    pub style: Style,
    /// The width of the element. Measured by the amount of characters in the content
    pub width: usize,
    selectable: bool,
}

impl RenderedElement {
    pub fn is_selected(&self, id: usize) -> bool {
        self.id == id && self.selectable
    }

    pub fn new(id: usize, content: impl Into<String>, style: impl Into<Style>) -> Self {
        let content: String = content.into();
        let width = content.chars().count();
        let style = style.into();
        RenderedElement {
            id,
            content,
            style,
            width,
            selectable: true,
        }
    }

    pub fn non_selectable(id: usize, content: impl Into<String>, style: impl Into<Style>) -> Self {
        let mut element = RenderedElement::new(id, content, style);
        element.selectable = false;
        element
    }
}

pub type Line = Vec<RenderedElement>;

pub struct RenderedPage {
    pub lines: Vec<Line>,
}

pub trait Renderer {
    fn render(page: &Page) -> RenderedPage;
    fn required_with(page: &Page) -> usize {
        0_usize
    }
}

pub struct TestRenderer;

impl Renderer for TestRenderer {
    fn render(page: &Page) -> RenderedPage {
        let amount_nodes = page.nodes.len();
        let mut lines = vec![
            vec![RenderedElement::non_selectable(
                0,
                "Test Page: Information about the current page",
                Style::title_primary(),
            )],
            vec![],
            vec![RenderedElement::non_selectable(
                1,
                "- the page has ".to_owned()
                    + &amount_nodes.to_string()
                    + " nodes that could be parsed",
                Style::primary(),
            )],
            vec![],
            vec![RenderedElement::non_selectable(
                0,
                "Parsed nodes:",
                Style::title_primary(),
            )],
        ];

        if let Some(node) = page.nth(0) {
            lines.push(vec![RenderedElement::non_selectable(
                node.raw().index,
                format!("- {:?}: '{:?}'", node.data(), node.raw()),
                Style::primary(),
            )]);
            node.descendants()
                .map(|descendant_node| {
                    lines.push(vec![RenderedElement::non_selectable(
                        descendant_node.raw().index,
                        format!(
                            "- {:?}: '{:?}'",
                            descendant_node.data(),
                            descendant_node.raw()
                        ),
                        Style::primary(),
                    )])
                })
                .count();
        }

        return RenderedPage { lines };
    }
}
