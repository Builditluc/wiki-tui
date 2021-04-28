pub mod wiki {
    pub mod search {
    	use serde::*;

        #[derive(Deserialize, Debug, Clone)]
        pub struct SearchResponse {
            #[serde(rename="continue")]
            pub continue_code: ContinueCode,
            pub query: QuerySearch
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct ContinueCode {
            #[serde(rename="continue")]
            pub continue_code: String,
            #[serde(rename="sroffset")]
            pub scroll_offset: i32
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct QuerySearch {
            pub search: Vec<SearchResult>,
            #[serde(rename="searchinfo")]
            pub search_info: SearchInfo
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct SearchResult {
            #[serde(rename="pageid")]
            pub page_id: i32,
            pub size: i32,
            pub snippet: String,
            pub timestamp: String,
            pub title: String,
            #[serde(rename="wordcount")]
            pub word_count: i32
        }

        #[derive(Deserialize, Debug, Clone)]
        pub struct SearchInfo {
            #[serde(rename="totalhits")]
            pub total_hits: i32
        }
    }
    pub mod article {
    	use serde::*;
        use cursive::utils::*;

        #[derive(Deserialize, Debug)]
        pub struct ArticleResponse {
            pub query: QueryArticle
        }

        #[derive(Deserialize, Debug)]
        pub struct QueryArticle {
            pub pages: Vec<ArticleResult>
        }

        #[derive(Deserialize, Debug)]
        pub struct ArticleResult {
            #[serde(rename="pageid")]
            pub page_id: i32,
            pub title: String,
            #[serde(rename="extract")]
            pub content: String
        }
        
        #[derive(Clone)]
        pub struct Article {
            pub title: String,
            pub paragraphs: Vec::<markup::StyledString>
        }
    }
    
    pub mod parser {
        pub struct Default;
    }
    pub struct ArticleResultPreview {
        pub page_id: i32,
        pub snippet: String,
        pub title: String,
   }

    pub struct Article {
        pub page_id: i32,
        pub title: String,
        pub content: String
   }
}

impl From<wiki::search::SearchResult> for wiki::ArticleResultPreview {
    fn from(search_result: wiki::search::SearchResult) -> Self {
        wiki::ArticleResultPreview {
            page_id: search_result.page_id,
            snippet: search_result.snippet,
            title: search_result.title
        }
    }
}

//impl From<wiki::article::ArticleResponse> for wiki::article::Article {
//    fn from(article_response: wiki::article::ArticleResponse) -> Self {
//        wiki::article::Article {
//            page_id: article_response.query.pages[0].page_id,
//            title: article_response.query.pages[0].title.to_string(),
//            content: article_response.query.pages[0].content.to_string()
//        }
//    }
//}

impl Parser for wiki::parser::Default {
    fn parse(&self, html: reqwest::blocking::Response) -> wiki::article::Article {
        use cursive::theme::*;
        use cursive::utils::*;
        use select::document::Document;
        use select::predicate::Class;

        log::info!("{:?}", html); 
        let mut paragraphs: Vec::<markup::StyledString> = Vec::new();
        let document = Document::from_read(html).unwrap();
        log::info!("Loaded the HTML document");
        // now iterate over all of the elements inside of the article
        for node in document.find(Class("mw-parser-output")) {
            log::info!("Iterating now over the node {:?}", node.name());
            for children in node.children() {
                // check, if the children is a html element
                if children.name().is_some() {
                    // match the name of the children
                    match children.name().unwrap() {
                        // if it's a header, make a new paragraph and add the header to it
                        "h2" | "h3" | "h4" | "h5" => {
                            let text = children.find(Class("mw-headline")).next().unwrap().text();
                            let mut styled_content = markup::StyledString::plain("\n");
                            styled_content.append_styled(text, Style::from(Color::Dark(BaseColor::Black)).combine(Effect::Bold));
                            styled_content.append_plain("\n\n");
                
                            paragraphs.push(styled_content);
                            log::info!("Added a headline to a new paragraph, there are now a total of {} paragraphs", paragraphs.len());
                        },
                        // if it's a paragraph, add it to the current paragraph
                        "p" => {
                            let text = children.text();

                            // get the current paragraph
                            let mut current_paragraph = match paragraphs.pop() {
                                Some(content) => content,
                                None => markup::StyledString::new()
                            }; 

                            current_paragraph.append_styled(text, Style::from(Color::Dark(BaseColor::Black)));
                            paragraphs.push(current_paragraph);
                            log::info!("Added some more text to the current paragraph, there are now a total of {} paragraphs", paragraphs.len());
                        },
                        // if it's a div with the class "reflist", add it to the current paragraph
                        // in form of a list
                        "div" if children.is(Class("reflist")) => {
                            let text = children.text();

                            // get the current paragraph
                            let mut current_paragraph = match paragraphs.pop() {
                                Some(content) => content,
                                None => markup::StyledString::new()
                            }; 

                            current_paragraph.append_styled(text, Style::from(Color::Dark(BaseColor::Black)));
                            paragraphs.push(current_paragraph);
                        },
                        // if it's any other html element, skip it
                        _ => continue
                    }
                }
            }
        }
        log::info!("a total of {} paragraphs were found", paragraphs.len());
        wiki::article::Article {
            title: String::new(),
            paragraphs,
        }
    }
}

pub trait Parser {
    fn parse(&self, html: reqwest::blocking::Response) -> wiki::article::Article;
}
