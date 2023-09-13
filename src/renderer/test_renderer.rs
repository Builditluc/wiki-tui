use ratatui::style::Style;
use wiki_api::document::{Document, Node};

use super::{RenderedDocument, Word};

#[derive(Clone, Debug)]
struct Descendants<'a> {
    start: Node<'a>,
    current: Node<'a>,
    done: bool,
    depth: usize,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = (Node<'a>, usize);

    fn next(&mut self) -> Option<(Node<'a>, usize)> {
        if self.done {
            return None;
        }

        // If this is the start, we can only descdend into children.
        if self.start.index() == self.current.index() {
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
                self.depth = self.depth.saturating_add(1);
            } else {
                self.done = true;
                return None;
            }
        } else {
            // Otherwise we can also go to next sibling.
            if let Some(first_child) = self.current.first_child() {
                self.depth = self.depth.saturating_add(1);
                self.current = first_child;
            } else if let Some(next) = self.current.next() {
                self.current = next;
            } else {
                loop {
                    // This unwrap should never fail.
                    let parent = self.current.parent().unwrap();
                    self.depth = self.depth.saturating_sub(1);
                    if parent.index() == self.start.index() {
                        self.done = true;
                        return None;
                    }
                    if let Some(next) = parent.next() {
                        self.current = next;
                        break;
                    }
                    self.current = parent;
                }
            }
        }

        Some((self.current, self.depth))
    }
}

pub fn render_tree_data<'a>(document: &'a Document) -> RenderedDocument {
    let mut lines: Vec<Vec<Word>> = Vec::new();

    let descendants = Descendants {
        start: document.nth(0).unwrap(),
        current: document.nth(0).unwrap(),
        done: false,
        depth: 0,
    };
    for (node, depth) in descendants {
        let content = format!("{}{:?}", " ".repeat(depth * 2), node.raw().data);
        lines.push(vec![Word {
            index: node.index(),
            content,
            style: Style::default(),
            width: 0.0,
            whitespace_width: 0.0,
            penalty_width: 0.0,
        }])
    }

    RenderedDocument { lines }
}

pub fn render_tree_raw<'a>(document: &'a Document) -> RenderedDocument {
    let mut lines: Vec<Vec<Word>> = Vec::new();

    let descendants = Descendants {
        start: document.nth(0).unwrap(),
        current: document.nth(0).unwrap(),
        done: false,
        depth: 0,
    };
    for (node, depth) in descendants {
        let content = format!("{}{:?}", " ".repeat(depth * 2), node.raw());
        lines.push(vec![Word {
            index: node.index(),
            content,
            style: Style::default(),
            width: 0.0,
            whitespace_width: 0.0,
            penalty_width: 0.0,
        }])
    }

    RenderedDocument { lines }
}

pub fn render_nodes_raw<'a>(document: &'a Document) -> RenderedDocument {
    let mut lines: Vec<Vec<Word>> = Vec::new();

    for raw in document.nodes.iter() {
        let content = format!("{:?}", raw);
        lines.push(vec![Word {
            index: raw.index,
            content,
            style: Style::default(),
            width: 0.0,
            whitespace_width: 0.0,
            penalty_width: 0.0,
        }])
    }

    RenderedDocument { lines }
}
