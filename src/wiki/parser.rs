use crate::wiki::article::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};

pub trait Parser {
    fn parse(&self, html: reqwest::blocking::Response) -> ParsedArticle;
}

pub struct Default;
impl Default {
    fn get_table_of_contents(
        &self,
        document: Document,
    ) -> Option<crate::ui::models::TableOfContents::Table> {
        use crate::ui::models::TableOfContents;

        let toc_html: Node;
        let mut toc_build = TableOfContents::Table {
            title: String::new(),
            items: Vec::new(),
        };

        log::info!("Parsing the table of contents now");
        if let Some(_toc_html) = document.find(Attr("id", "toc")).next() {
            toc_html = _toc_html;
        } else {
            log::warn!("Couldn't find the table of contents");
            return None;
        }

        toc_build.title = toc_html.find(Class("toctitle")).next().unwrap().text();

        if let Some(toc_items) = toc_html.find(Name("ul")).next() {
            log::info!("Now parsing the content of the table of contents");
            for toc_item in toc_items.find(Name("li")) {
                toc_build.items.push(self.parse_toc_item(toc_item, 0));
            }
        } else {
            log::warn!("No content was found in the table of contents");
            return None;
        }

        log::info!("Sucessfully build the table of contents");
        log::debug!("TableOfContents: \n{:?}", toc_build);
        Some(toc_build)
    }

    fn parse_toc_item(&self, item: Node, level: i32) -> crate::ui::models::TableOfContents::Item {
        let mut item_build = crate::ui::models::TableOfContents::Item {
            number: level,
            text: String::new(),
            sub_items: None,
        };

        let item_number = item.find(Class("tocnumber")).next().unwrap().text();
        let item_text = item.find(Class("toctext")).next().unwrap().text();
        item_build.text = format!("{} {}", item_number, item_text);

        if let Some(_sub_items) = item.find(Name("ul")).next() {
            let mut sub_items = Vec::new();
            for sub_item in _sub_items.find(Name("li")) {
                sub_items.push(self.parse_toc_item(sub_item, level + 1));
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
        item_build
    }
}

impl Parser for Default {
    fn parse(&self, html: reqwest::blocking::Response) -> ParsedArticle {
        let mut content: Vec<ArticleElement> = Vec::new();
        let document = Document::from_read(html).unwrap();
        log::info!("Loaded the HTML document");
        log::info!("The Article will now be parsed");

        // add the title to the article content
        let title = document.find(Class("firstHeading")).next().unwrap().text();
        content.push(ArticleElement {
            content: title,
            element_type: ArticleElementType::Header,
            link_target: None,
        });

        // now iterate over all of the elements inside of the article
        for node in document.find(Class("mw-parser-output")) {
            log::debug!("Iterating now over the node {:?}", node.name());
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
                            log::debug!("Added a headline to the article content");
                        }
                        // if it's a paragraph, add it to the context with only ONE Linebreak at
                        // the end
                        "p" => {
                            content.append(&mut self.parse_child(children));
                            log::debug!("Added a paragraph to the article content");
                        }
                        // if it's a div with the class "reflist", add it to the current paragraph
                        // in form of a list
                        "div" if children.is(Class("reflist")) => {
                            log::debug!("Added the Reference List to the article content");
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
                            log::debug!("Added a list to the article content");
                        }
                        // if it's any other html element, skip it
                        _ => continue,
                    }
                }
            }
        }

        let toc = self.get_table_of_contents(document);

        log::info!("Finished parsing the article");
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
            log::debug!("Iterating now over the node {:?}", element.name());

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
