use crate::wiki::article::Article;

pub trait Parser {
    fn parse(&self, html: reqwest::blocking::Response) -> Article;
}

pub struct Default;
impl Parser for Default {
    fn parse(&self, html: reqwest::blocking::Response) -> Article {
        use cursive::theme::*;
        use cursive::utils::*;
        use select::document::Document;
        use select::predicate::Class;

        let mut content = markup::StyledString::new();
        let document = Document::from_read(html).unwrap();
        log::info!("[wiki::parser::Default::parse] Loaded the HTML document");

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
                            let mut styled_content = markup::StyledString::new();
                            styled_content.append_plain("\n");
                            styled_content.append_styled(
                                text,
                                Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold),
                            );
                            styled_content.append_plain("\n\n");

                            content.append(styled_content);
                            log::info!("[wiki::parser::Default::parse] Added a headline to the article content");
                        }
                        // if it's a paragraph, add it to the context with only ONE Linebreak at
                        // the end
                        "p" => {
                            let text = children.text();

                            let styled_text = markup::StyledString::plain(text);

                            content.append(styled_text);
                            log::info!("[wiki::parser::Default::parse] Added a paragraph to the article content");
                        }
                        // if it's a div with the class "reflist", add it to the current paragraph
                        // in form of a list
                        "div" if children.is(Class("reflist")) => {
                            let text = children.text();
                            let styled_text = markup::StyledString::plain(text);

                            content.append(styled_text);
                            log::info!("[wiki::parser::Default::parse] Added the Reference List to the article content");
                        }
                        // if it's any other html element, skip it
                        _ => continue,
                    }
                }
            }
        }
        log::info!("[wiki::parser::Default::parse] Finished parsing the article");
        Article {
            title: String::new(),
            content,
        }
    }
}
