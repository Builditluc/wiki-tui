use std::io::Read;

use crate::config::{ParserConfig, CONFIG};
use crate::wiki::article::{
    compiled_article::Article,
    element::ArticleElement,
    toc::{TableOfContents, TableOfContentsItem},
};

use anyhow::{Context, Result};
use cursive::theme::{Effect, Style};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

pub trait Parser<'a> {
    fn new(config: &'a ParserConfig) -> Self;
    fn parse<R: Read>(&mut self, html: R) -> Result<Article>;
}

pub struct DefaultParser<'a> {
    config: &'a ParserConfig,
    elements: Vec<ArticleElement>,
}

impl<'a> DefaultParser<'a> {
    fn parse_toc(&self, document: &Document) -> Result<Option<TableOfContents>> {
        let toc_node = document
            .find(Attr("id", "toc"))
            .next()
            .context("No table of contents was found")?;
        let toc_title = toc_node
            .find(Class("toctitle"))
            .next()
            .context("No toc title was found")?
            .text();
        let mut toc_items: Vec<TableOfContentsItem> = Vec::new();

        for node in toc_node
            .find(Name("ul"))
            .next()
            .context("No items were found inside of the table of contents")?
            .find(Name("li"))
        {
            if let Ok(item) = self.parse_toc_item(node, 0) {
                toc_items.push(item);
                continue;
            }
        }

        Ok(Some(TableOfContents::new(toc_title, toc_items)))
    }

    fn parse_toc_item(&self, node: Node, level: i32) -> Result<TableOfContentsItem> {
        let item_number = node
            .find(Class("tocnumber"))
            .next()
            .context("Couldn't find the number for the current item")?
            .text();
        let item_text = node
            .find(Class("toctext"))
            .next()
            .context("Couldn't find the text for the current item")?
            .text();
        let mut sub_items: Vec<TableOfContentsItem> = Vec::new();

        if let Some(items) = node.find(Name("ul")).next() {
            for item in items.find(Name("li")) {
                if let Ok(parsed_item) = self.parse_toc_item(item, level + 1) {
                    sub_items.push(parsed_item);
                    continue;
                }
            }
        }

        Ok(TableOfContentsItem::new(
            level,
            format!("{} {}", item_number, item_text),
            {
                if sub_items.is_empty() {
                    None
                } else {
                    Some(sub_items)
                }
            },
        ))
    }

    fn parse_node(&mut self, node: Node) {
        match node.name().unwrap_or_default() {
            "h2" | "h3" | "h4" | "h5" if self.config.headers => {
                if let Some(headline_node) = node.find(Class("mw-headline")).next() {
                    self.push_header(headline_node.text())
                }
            }
            "b" => self.push_text(
                node.text(),
                Some(Style::from(CONFIG.theme.text).combine(Effect::Bold)),
            ),
            "i" => self.push_text(
                node.text(),
                Some(Style::from(CONFIG.theme.text).combine(Effect::Italic)),
            ),
            "a" => {
                let content = node.text();
                if let Some(target) = node.attr("href") {
                    self.push_link(content, target);
                    return;
                }
                self.push_text(content, None);
            }
            "p" if self.config.paragraphs => {
                for child in node.children() {
                    self.parse_node(child)
                }
            }
            "ul" if self.config.lists => {
                for list_item in node
                    .children()
                    .filter(|node| node.name().unwrap_or_default() == "li")
                {
                    self.push_newline();
                    self.push_text("\t- ".to_string(), None);
                    for child in list_item.children() {
                        self.parse_node(child)
                    }
                    self.push_newline();
                }
            }
            "pre" if self.config.code_blocks => {
                self.push_newline();
                if let Some(code_node) = node.find(Name("code")).next() {
                    for child in code_node.children() {
                        self.parse_node(child)
                    }
                }
                self.push_newline();
            }
            _ => {
                if let Some(text) = node.as_text() {
                    self.push_text(text.to_string(), None)
                }
            }
        }
    }

    fn push_link(&mut self, content: String, target: &str) {
        self.elements.push(
            ArticleElement::new(
                self.get_id(),
                content.chars().count(),
                Style::from(CONFIG.theme.text).combine(Effect::Underline),
                content,
            )
            .attribute("type", "link")
            .attribute("target", target),
        );
    }

    fn push_text(&mut self, content: String, style: Option<Style>) {
        self.elements.push(ArticleElement::new(
            self.get_id(),
            content.chars().count(),
            style.unwrap_or_else(|| Style::from(CONFIG.theme.text)),
            content,
        ))
    }

    fn push_header(&mut self, content: String) {
        self.push_newline();
        self.elements.push(
            ArticleElement::new(
                self.elements.len() as i32,
                content.chars().count(),
                Style::from(CONFIG.theme.title).combine(Effect::Bold),
                content,
            )
            .attribute("type", "header"),
        );
        self.push_newline();
        self.push_newline();
    }

    fn push_newline(&mut self) {
        self.elements.push(ArticleElement::newline(self.get_id()));
    }

    fn get_id(&self) -> i32 {
        self.elements.len() as i32
    }
}

impl<'a> Parser<'a> for DefaultParser<'a> {
    fn new(config: &'a ParserConfig) -> Self {
        Self {
            config,
            elements: Vec::new(),
        }
    }

    fn parse<R: Read>(&mut self, html: R) -> Result<Article> {
        let document = Document::from_read(html).context("Failed reading the response")?;

        let title = document
            .find(Class("mw-first-heading"))
            .next()
            .context("Couldn't find the title")?
            .text();
        self.push_header(title);

        document
            .find(Class("mw-parser-output"))
            .next()
            .context("Couldn't find the content of the article")?
            .children()
            .map(|child| self.parse_node(child))
            .count();

        let toc = self.parse_toc(&document).unwrap_or_default();

        Ok(Article::new(std::mem::take(&mut self.elements), toc))
    }
}

#[cfg(test)]
mod tests {
    use super::{ArticleElement, DefaultParser, Parser};
    use crate::config::{ParserConfig, CONFIG};
    use cursive::theme::{Effect, Style};

    fn build_parser_config() -> ParserConfig {
        ParserConfig {
            toc: true,
            headers: true,
            paragraphs: true,
            lists: true,
            code_blocks: true,
        }
    }

    fn generate_html(html: &str) -> String {
        format!(
            "<html><body><div id=\"content\"><div id=\"bodyContent\"><div id=\"mw-content-text\"><div class=\"mw-parser-output\">{}</div></div></div></div></body></html>",
            html
        )
    }

    #[test]
    fn parse_link() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html = generate_html(
            "<h1 class=\"mw-first-heading\">Github</h1><p><a href=\"/wiki/Software_development\">software development</a></p>",
        );
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &4).next().unwrap(),
            &ArticleElement::new(
                4,
                20,
                Style::from(CONFIG.theme.text).combine(Effect::Underline),
                "software development".to_string(),
            )
            .attribute("type", "link")
            .attribute("target", "/wiki/Software_development")
        );
    }

    #[test]
    fn parse_text() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html =
            generate_html("<h1 class=\"mw-first-heading\">Github</h1><p>is a provider of</p>");
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &4).next().unwrap(),
            &ArticleElement::new(
                4,
                16,
                Style::from(CONFIG.theme.text),
                "is a provider of".to_string(),
            )
        );
    }

    #[test]
    fn parse_header() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html = generate_html(
            "<h1 class=\"mw-first-heading\">Github</h1><h2><span class=\"mw-headline\">History</span></h2>",
        );
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &5).next().unwrap(),
            &ArticleElement::new(
                5,
                7,
                Style::from(CONFIG.theme.title).combine(Effect::Bold),
                "History".to_string(),
            )
            .attribute("type", "header")
        );
    }

    #[test]
    fn parse_bold() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html =
            generate_html("<h1 class=\"mw-first-heading\">Github</h1><p><b>GitHub, Inc.</b></p>");
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &4).next().unwrap(),
            &ArticleElement::new(
                4,
                12,
                Style::from(CONFIG.theme.text).combine(Effect::Bold),
                "GitHub, Inc.".to_string(),
            )
        );
    }

    #[test]
    fn parse_italic() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html =
            generate_html("<h1 class=\"mw-first-heading\">Github</h1><p><i>GitHub, Inc.</i></p>");
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &4).next().unwrap(),
            &ArticleElement::new(
                4,
                12,
                Style::from(CONFIG.theme.text).combine(Effect::Italic),
                "GitHub, Inc.".to_string(),
            )
        );
    }

    #[test]
    fn parse_list() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html = generate_html(
            "<h1 class=\"mw-first-heading\">Github</h1><ul><li>Documentation,<a href=\"/wiki/README\">README</a></li></ul>",
        );
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &5).next().unwrap(),
            &ArticleElement::new(
                5,
                "\t- ".chars().count(),
                Style::from(CONFIG.theme.text),
                "\t- ".to_string(),
            )
        );

        assert_eq!(
            article.elements().filter(|x| x.id() == &7).next().unwrap(),
            &ArticleElement::new(
                7,
                6,
                Style::from(CONFIG.theme.text).combine(Effect::Underline),
                "README".to_string(),
            )
            .attribute("type", "link")
            .attribute("target", "/wiki/README")
        );
    }

    #[test]
    fn parse_code_block() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html = generate_html(
            "<h1 class=\"mw-first-heading\">Github</h1><pre><code>inverse(a, n) t := 0</code></pre>",
        );
        let article = parser.parse(test_html.as_bytes()).unwrap();

        assert_eq!(
            article.elements().filter(|x| x.id() == &5).next().unwrap(),
            &ArticleElement::new(
                5,
                20,
                Style::from(CONFIG.theme.text),
                "inverse(a, n) t := 0".to_string(),
            )
        );
    }

    #[test]
    fn incorrect_html() {
        let config = build_parser_config();
        let mut parser = DefaultParser::new(&config);

        let test_html = generate_html("nope");
        assert!(parser.parse(test_html.as_bytes()).is_err())
    }
}
