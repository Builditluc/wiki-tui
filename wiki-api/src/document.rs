use serde_repr::Deserialize_repr;

use crate::page::Link;

#[derive(Clone, PartialEq, Eq)]
pub struct Document {
    pub nodes: Vec<Raw>,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nodes: {}", self.nodes.len())
    }
}

impl Document {
    pub fn nth(&self, n: usize) -> Option<Node> {
        Node::new(self, n)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr)]
#[repr(usize)]
pub enum HeaderKind {
    Main = 1,
    Sub = 2,
    Section = 3,
    Subsection = 4,
    Minor = 5,
    Detail = 6,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum Data {
    Section {
        id: usize,
    },
    Header {
        id: String,
        kind: HeaderKind,
    },
    Text {
        contents: String,
    },
    Division,
    Paragraph,
    Span,
    Reflink,
    Hatnote,
    RedirectMessage,
    Disambiguation,
    Blockquote,

    OrderedList,
    UnorderedList,
    ListItem,

    DescriptionList,
    DescriptionListTerm,
    DerscriptionListDescription,

    Bold,
    Italic,

    Linebreak,

    Link(Link),
    #[default]
    Unknown,

    Unsupported(UnsupportedElement),
    UnsupportedInline(UnsupportedElement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnsupportedElement {
    Table,
    Image,
    Figure,
    MathElement,
    PreformattedText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raw {
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub first_child: Option<usize>,
    pub last_child: Option<usize>,
    pub data: Data,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Node<'a> {
    document: &'a Document,
    index: usize,
}

impl<'a> Node<'a> {
    pub fn new(document: &'a Document, index: usize) -> Option<Node<'a>> {
        if index < document.nodes.len() {
            Some(Node { document, index })
        } else {
            None
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn raw(&self) -> &'a Raw {
        &self.document.nodes[self.index]
    }

    pub fn data(&self) -> &'a Data {
        &self.raw().data
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.raw()
            .parent
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.raw()
            .prev
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.raw()
            .next
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn first_child(&self) -> Option<Node<'a>> {
        self.raw()
            .first_child
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn last_child(&self) -> Option<Node<'a>> {
        self.raw()
            .last_child
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn descendants(&self) -> Descendants<'a> {
        Descendants {
            start: *self,
            current: *self,
            done: false,
        }
    }

    pub fn children(&self) -> Children<'a> {
        Children {
            next: self.first_child(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Descendants<'a> {
    start: Node<'a>,
    current: Node<'a>,
    done: bool,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        if self.done {
            return None;
        }

        // If this is the start, we can only descdend into children.
        if self.start.index() == self.current.index() {
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else {
                self.done = true;
                return None;
            }
        } else {
            // Otherwise we can also go to next sibling.
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else if let Some(next) = self.current.next() {
                self.current = next;
            } else {
                loop {
                    // This unwrap should never fail.
                    let parent = self.current.parent().unwrap();
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

        Some(self.current)
    }
}

pub struct Children<'a> {
    next: Option<Node<'a>>,
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Node<'a>> {
        if let Some(next) = self.next {
            self.next = next.next();
            Some(next)
        } else {
            None
        }
    }
}
