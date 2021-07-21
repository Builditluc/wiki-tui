use crate::wiki::article::*;
use select::document::Document;
use select::node::Node;
use select::predicate::Class;

pub trait Parser {
    fn parse(&self, html: reqwest::blocking::Response) -> ParsedArticle;
}

pub struct Default;
impl Default {
    fn get_table_of_contents(
        &self,
        document: Document,
    ) -> Option<crate::ui::models::TableOfContents::Table> {
        None
    }
}

impl Parser for Default {
    fn parse(&self, html: reqwest::blocking::Response) -> ParsedArticle {
        let mut content: Vec<ArticleElement> = Vec::new();
        let document = Document::from_read(html).unwrap();
        log::info!("[wiki::parser::Default::parse] Loaded the HTML document");

        // add the title to the article content
        let title = document.find(Class("firstHeading")).next().unwrap().text();
        content.push(ArticleElement {
            content: title,
            element_type: ArticleElementType::Header,
            link_target: None,
        });

        // now iterate over all of the elements inside of the article
        for node in document.find(Class("mw-parser-output")) {
            log::info!(
                "[wiki::parser::Default::parse] Iterating now over the node {:?}",
                node.name()
            );
            for children in node.children() {
                // check, if the children is a html element
                if children.name().is_some() {
                    // match the name of the children
                    match children.name().unwrap() {
                        // if it's a header, add it to the article content in BOLD and with two
                        // Linebreaks at the end
                        "h2" | "h3" | "h4" | "h5" => {
                            let text = children.find(Class("mw-headline")).next().unwrap().text();

                            content.push(ArticleElement {
                                content: text,
                                element_type: ArticleElementType::Header,
                                link_target: None,
                            });
                            log::info!("[wiki::parser::Default::parse] Added a headline to the article content");
                        }
                        // if it's a paragraph, add it to the context with only ONE Linebreak at
                        // the end
                        "p" => {
                            content.append(&mut self.parse_child(children));
                            log::info!("[wiki::parser::Default::parse] Added a paragraph to the article content");
                        }
                        // if it's a div with the class "reflist", add it to the current paragraph
                        // in form of a list
                        "div" if children.is(Class("reflist")) => {
                            log::info!("[wiki::parser::Default::parse] Added the Reference List to the article content");
                        }
                        // if it's a list, add every element to the current paragraph
                        "ul" => {
                            let mut list_string = "".to_string();
                            // go through every element in the list and add it
                            for element in children.children() {
                                if element.name().unwrap_or("") == "li" {
                                    list_string = list_string + "\t- " + &element.text() + "\n";
                                }
                            }
                            content.push(ArticleElement {
                                content: list_string,
                                element_type: ArticleElementType::Text,
                                link_target: None,
                            });
                            log::info!("[wiki::parser::Default::parse] Added a list to the article content");
                        }
                        // if it's any other html element, skip it
                        _ => continue,
                    }
                }
            }
        }

        // TODO: get the table of contents
        let toc = self.get_table_of_contents(document);

        log::info!("[wiki::parser::Default::parse] Finished parsing the article");
        ParsedArticle {
            article: Article { elements: content },
            toc,
        }
    }
}

impl Default {
    fn parse_child(&self, element: Node) -> Vec<ArticleElement> {
        let mut content: Vec<ArticleElement> = Vec::new();

        // go through every elements inside of the element
        for children in element.children() {
            log::info!(
                "[wiki::parser::Default::parse_child] Iterating now over the node {:?}",
                element.name()
            );

            match children.name().unwrap_or_else(|| "") {
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
