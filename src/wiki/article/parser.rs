use crate::config::{ParserConfig, CONFIG};
use crate::wiki::article::{
    compiled_article::Article,
    element::ArticleElement,
    toc::{TableOfContents, TableOfContentsItem},
};

use anyhow::{Context, Result};
use cursive::theme::{Effect, Style};
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name},
};
use std::io::Read;

/// The Parser trait allows for generating an Article from a html source
pub trait Parser<'a> {
    fn new(config: &'a ParserConfig) -> Self;
    fn parse<R: Read>(&mut self, html: R) -> Result<Article>;
}

/// The Default Parser. It can generate an Article from a given html source. Requires a
/// configuration
pub struct DefaultParser<'a> {
    /// The configuration of the Parser
    config: &'a ParserConfig,
    /// The elements that have been parsed already
    elements: Vec<ArticleElement>,
}

impl<'a> DefaultParser<'a> {
    /// This functinon takes generates a TableOfContents from a given document. When no
    /// TableOfContents can be found in the document, it returns Ok(None). Any errors it
    /// encounters are returned
    fn parse_toc(&self, document: &Document) -> Result<Option<TableOfContents>> {
        log::debug!("parse_toc was called");

        // get the toc node from the document if it exists
        log::debug!("retrieving the required nodes from the document");
        let toc_node = document
            .find(Attr("id", "toc"))
            .next()
            .context("No table of contents was found")?;

        // get the title of the toc
        let toc_title = toc_node
            .find(Class("toctitle"))
            .next()
            .context("No toc title was found")?
            .text();

        log::debug!("parsing the toc now");
        let mut toc_items: Vec<TableOfContentsItem> = Vec::new();

        // parse every child of the toc node
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

        log::debug!("parse_toc finished successfully");
        Ok(Some(TableOfContents::new(toc_title, toc_items)))
    }

    /// A helper function that parses a single node from a html document into a
    /// TableOfContentsItem. Any errors it encounters are returned
    fn parse_toc_item(&self, node: Node, level: i32) -> Result<TableOfContentsItem> {
        // get the item number
        let item_number = node
            .find(Class("tocnumber"))
            .next()
            .context("Couldn't find the number for the current item")?
            .text();

        // get the text
        let item_text = node
            .find(Class("toctext"))
            .next()
            .context("Couldn't find the text for the current item")?
            .text();

        // if there are any sub items, parse them
        let mut sub_items: Vec<TableOfContentsItem> = Vec::new();
        if let Some(items) = node.find(Name("ul")).next() {
            for item in items.find(Name("li")) {
                if let Ok(parsed_item) = self.parse_toc_item(item, level + 1) {
                    sub_items.push(parsed_item);
                    continue;
                }
            }
        }

        // return everything
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

    /// A helper function that parses a single node from a html document into one or multiple
    /// ArticleElement's and just adds them to the elements array
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
                // if the node has a link, then push the link
                // else, just add it as normal text
                if let Some(target) = node.attr("href") {
                    self.push_link(content, target);
                    return;
                }
                self.push_text(content, None);
            }
            "p" if self.config.paragraphs => {
                // parse every child node of the paragraph
                for child in node.children() {
                    self.parse_node(child)
                }
                // after every paragraph we want a newline
                self.push_newline()
            }
            "ul" if self.config.lists => {
                // go through every list item inside of the node
                for list_item in node
                    .children()
                    .filter(|node| node.name().unwrap_or_default() == "li")
                {
                    // add a newline and a tab at the beginning of the line and
                    // parse every child node of the list item
                    self.push_newline();
                    self.push_text("\t- ".to_string(), None);
                    for child in list_item.children() {
                        self.parse_node(child)
                    }
                }
                // after every list we want a newline
                self.push_newline()
            }
            "pre" if self.config.code_blocks => {
                // for the code blocks, we just parse it like normal but add a newline at the
                // beginning and the end
                self.push_newline();
                if let Some(code_node) = node.find(Name("code")).next() {
                    for child in code_node.children() {
                        self.parse_node(child)
                    }
                }
                self.push_newline();
            }
            _ => {
                // only if the node is raw text, we add it. we wont add any other nodes
                if let Some(text) = node.as_text() {
                    self.push_text(text.to_string(), None)
                }
            }
        }
    }

    /// A helper function that adds a new link to the elements. It constructs an ArticleElement
    /// from the given content and target and then adds it to the array
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

    /// A helper function that adds normal, optionally styled text to the elements. It constructs an
    /// ArticleElement from the given content and optional style and adds it to the array
    fn push_text(&mut self, content: String, style: Option<Style>) {
        // if the content has a newline inside of it, we replace the newline with actual newlines
        // by splitting the content
        if content.contains('\n') {
            for span in content.lines() {
                self.elements.push(ArticleElement::new(
                    self.get_id(),
                    span.chars().count(),
                    style.unwrap_or_else(|| Style::from(CONFIG.theme.text)),
                    span.to_string(),
                ));

                self.push_newline();
            }

            // remove the last newline
            self.elements.pop();
            return;
        }

        // if there are no newlines in the content, we can just create and add a new
        // ArticleElement
        self.elements.push(ArticleElement::new(
            self.get_id(),
            content.chars().count(),
            style.unwrap_or_else(|| Style::from(CONFIG.theme.text)),
            content,
        ))
    }

    /// A helper function that add a header to the elements. It constructs an ArticleElement from
    /// a given content and adds it to the array
    fn push_header(&mut self, content: String) {
        // we create a new article element with the correct style from the config and add a newline
        // afterwards
        self.elements.push(
            ArticleElement::new(
                self.get_id(),
                content.chars().count(),
                Style::from(CONFIG.theme.title).combine(Effect::Bold),
                content,
            )
            .attribute("type", "header"),
        );
        self.push_newline();
    }

    /// A helper function that adds a newline to the elements
    fn push_newline(&mut self) {
        self.elements.push(ArticleElement::newline(self.get_id()));
    }

    /// A helper function that generates a new id for an element
    fn get_id(&self) -> i32 {
        self.elements.len() as i32
    }
}

impl<'a> Parser<'a> for DefaultParser<'a> {
    /// Creates a new DefaultParser with the given ParserConfig
    fn new(config: &'a ParserConfig) -> Self {
        log::debug!("creating a new instance of DefaultParser");
        Self {
            config,
            elements: Vec::new(),
        }
    }

    /// Tries to parse a given html document into an Article. Any errors it encounters will be
    /// returned
    fn parse<R: Read>(&mut self, html: R) -> Result<Article> {
        log::debug!("parse was called");

        // load the document
        let document = Document::from_read(html).context("failed reading the document")?;
        log::debug!("loaded the document");

        // retrieve the title of the article
        let title = document
            .find(Class("mw-first-heading"))
            .next()
            .context("Couldn't find the title")?
            .text();
        log::debug!("retrieved the title '{}' from the document", &title);
        self.push_header(title);

        // parse the article content
        document
            .find(Class("mw-parser-output"))
            .next()
            .context("Couldn't find the content of the article")?
            .children()
            .map(|child| self.parse_node(child))
            .count();

        // parse the table of contents (if it exists)
        let toc = match self.parse_toc(&document) {
            Ok(toc) => toc,
            Err(error) => {
                log::warn!("{}", error);
                None
            }
        };

        log::debug!("parse finished successfully");
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
            article.elements().filter(|x| x.id() == &2).next().unwrap(),
            &ArticleElement::new(
                2,
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
            article.elements().filter(|x| x.id() == &2).next().unwrap(),
            &ArticleElement::new(
                2,
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
            article.elements().filter(|x| x.id() == &2).next().unwrap(),
            &ArticleElement::new(
                2,
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
            article.elements().filter(|x| x.id() == &2).next().unwrap(),
            &ArticleElement::new(
                2,
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
            article.elements().filter(|x| x.id() == &2).next().unwrap(),
            &ArticleElement::new(
                2,
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
            article.elements().filter(|x| x.id() == &4).next().unwrap(),
            &ArticleElement::new(
                4,
                "\t- ".chars().count(),
                Style::from(CONFIG.theme.text),
                "\t- ".to_string(),
            )
        );

        assert_eq!(
            article.elements().filter(|x| x.id() == &6).next().unwrap(),
            &ArticleElement::new(
                6,
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
            article.elements().filter(|x| x.id() == &3).next().unwrap(),
            &ArticleElement::new(
                3,
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
