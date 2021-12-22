use crate::config::CONFIG;
use crate::ui;
use crate::wiki::article::*;

use anyhow::{Context, Result};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

pub trait Parser {
    fn parse(&self, html: reqwest::blocking::Response) -> Result<ParsedArticle>;
}

pub struct Default;
impl Default {
    fn get_table_of_contents(
        &self,
        document: Document,
    ) -> Result<Option<ui::models::table_of_contents::Table>> {
        let toc_html: Node;
        let mut toc_build = ui::models::table_of_contents::Table {
            title: String::new(),
            items: Vec::new(),
        };

        log::debug!("Parsing the table of contents now");
        if let Some(_toc_html) = document.find(Attr("id", "toc")).next() {
            toc_html = _toc_html;
        } else {
            log::warn!("Couldn't find the table of contents");
            return Ok(None);
        }

        toc_build.title = toc_html
            .find(Class("toctitle"))
            .next()
            .context("Couldn't find the table of contents title")?
            .text();

        let toc_items = toc_html
            .find(Name("ul"))
            .next()
            .context("Couldn't find the items of the table of contents")?;

        log::debug!("Now parsing the content of the table of contents");
        for toc_item_html in toc_items.find(Name("li")) {
            let toc_item = match self.parse_toc_item(toc_item_html, 0) {
                Ok(toc_item) => toc_item,
                Err(error) => {
                    log::warn!("{:?}", error);
                    continue;
                }
            };
            toc_build.items.push(toc_item);
        }

        log::debug!("Sucessfully build the table of contents");
        log::trace!("TableOfContents: \n{:?}", toc_build);
        Ok(Some(toc_build))
    }

    fn parse_toc_item(
        &self,
        item: Node,
        level: i32,
    ) -> Result<ui::models::table_of_contents::Item> {
        let mut item_build = ui::models::table_of_contents::Item {
            number: level,
            text: String::new(),
            sub_items: None,
        };

        let item_number = item
            .find(Class("tocnumber"))
            .next()
            .context("Couldn't find the number for the current item")?
            .text();
        let item_text = item
            .find(Class("toctext"))
            .next()
            .context("Couldn't find the text for the current item")?
            .text();

        item_build.text = format!("{} {}", item_number, item_text);

        if let Some(_sub_items) = item.find(Name("ul")).next() {
            let mut sub_items = Vec::new();
            for sub_item_html in _sub_items.find(Name("li")) {
                let sub_item = match self.parse_toc_item(sub_item_html, level + 1) {
                    Ok(sub_item) => sub_item,
                    Err(error) => {
                        log::warn!("{:?}", error);
                        continue;
                    }
                };

                sub_items.push(sub_item);
            }
            log::debug!(
                "A total of {} sub items were found in the item {}",
                sub_items.len(),
                item_text
            );
            item_build.sub_items = Some(sub_items);
        }

        log::debug!(
            "Sucessfully parsed the table of contents item {}",
            item_text
        );
        Ok(item_build)
    }
}

impl Parser for Default {
    fn parse(&self, html: reqwest::blocking::Response) -> Result<ParsedArticle> {
        let mut content: Vec<ArticleElement> = Vec::new();
        let document = Document::from_read(html).unwrap();
        log::debug!("Loaded the HTML document");
        log::debug!("The Article will now be parsed");

        // add the title to the article content
        let title = document
            .find(Class("firstHeading"))
            .next()
            .context("Coulnd't find the title")?
            .text();

        content.push(ArticleElement {
            content: title,
            element_type: ArticleElementType::Header,
            link_target: None,
        });
        log::trace!("Using this configuration: {:?}", CONFIG.parser);

        // TODO: improve this
        // now iterate over all of the elements inside of the article
        for node in document.find(Class("mw-parser-output")) {
            for children in node.children() {
                if children.name().is_none() {
                    continue;
                }

                match children.name().unwrap() {
                    // if it's a header, add it to the article content in BOLD and with two
                    // Linebreaks at the end
                    "h2" | "h3" | "h4" | "h5" if CONFIG.parser.headers => {
                        let text = children
                            .find(Class("mw-headline"))
                            .next()
                            .context(
                                format!(
                                    "Couldn't find a headline in the current element\nThe Node is: {:#?}", node.html()
                                )
                            )?
                            .text();

                        content.push(ArticleElement {
                            content: text,
                            element_type: ArticleElementType::Header,
                            link_target: None,
                        });
                        log::trace!("Added a headline to the article content");
                    }
                    // if it's a paragraph, add it to the context with only ONE Linebreak at
                    // the end
                    "p" if CONFIG.parser.paragraphs => {
                        content.append(&mut self.parse_child(children));
                        log::trace!("Added a paragraph to the article content");
                    }
                    // if it's a list, add every element to the current paragraph
                    "ul" if CONFIG.parser.lists => {
                        for element in children.children() {
                            if element.name().unwrap_or("") == "li" {
                                content.push(ArticleElement {
                                    content: format!("\t- {}\n", &element.text()),
                                    element_type: ArticleElementType::Text,
                                    link_target: None,
                                });
                            }
                        }
                        log::trace!("Added a list to the article content");
                    }
                    "pre" if CONFIG.parser.code_blocks => {
                        content.push(ArticleElement {
                            content: "\n".to_string(),
                            element_type: ArticleElementType::Text,
                            link_target: None,
                        });
                        content.append(&mut self.parse_child(children));
                        content.push(ArticleElement {
                            content: "\n".to_string(),
                            element_type: ArticleElementType::Text,
                            link_target: None,
                        });

                        log::trace!("Added a code block to the article content");
                    }
                    // if it's any other html element, skip it
                    _ => continue,
                }
            }
        }

        let toc = match self.get_table_of_contents(document) {
            Ok(toc) if CONFIG.parser.toc => toc,
            Err(error) => {
                log::warn!("{:?}", error);
                None
            }
            _ => None,
        };

        log::debug!("Finished parsing the article");
        Ok(ParsedArticle {
            article: Article { elements: content },
            toc,
        })
    }
}

impl Default {
    fn parse_child(&self, element: Node) -> Vec<ArticleElement> {
        let mut content: Vec<ArticleElement> = Vec::new();

        // go through every elements inside of the element
        for children in element.children() {
            log::trace!("Iterating now over the node {:?}", element.name());

            match children.name().unwrap_or("") {
                "a" => content.push(ArticleElement {
                    content: children.text(),
                    element_type: ArticleElementType::Link,
                    link_target: children.attr("href").map(str::to_string),
                }),
                "b" => content.push(ArticleElement {
                    content: children.text(),
                    element_type: ArticleElementType::Bold,
                    link_target: None,
                }),
                "i" => content.push(ArticleElement {
                    content: children.text(),
                    element_type: ArticleElementType::Italic,
                    link_target: None,
                }),
                "" => content.push(ArticleElement {
                    content: children.text(),
                    element_type: ArticleElementType::Text,
                    link_target: None,
                }),
                _ => continue,
            }
        }

        content
    }
}
