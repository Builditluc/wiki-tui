mod builder;
mod compiled_article;
mod element;
pub mod parser;
mod toc;

pub type Article = compiled_article::Article;
pub type ArticleElement = element::ArticleElement;
pub type ArticleBuilder = builder::ArticleBuilder;

pub type TableOfContents = toc::TableOfContents;
pub type TableOfContentsItem = toc::TableOfContentsItem;
