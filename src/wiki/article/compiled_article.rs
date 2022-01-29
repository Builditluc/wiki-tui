use crate::wiki::article::{element::ArticleElement, toc::TableOfContents};

#[derive(PartialEq, Debug)]
pub struct Article {
    elements: Vec<ArticleElement>,
    toc: Option<TableOfContents>,
}

impl Article {
    pub fn new(elements: Vec<ArticleElement>, toc: Option<TableOfContents>) -> Self {
        Self { elements, toc }
    }
    pub fn elements(&self) -> impl Iterator<Item = &ArticleElement> {
        self.elements.iter()
    }
    pub fn toc(&self) -> Option<&TableOfContents> {
        self.toc.as_ref()
    }
}
