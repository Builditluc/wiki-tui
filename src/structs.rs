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
    }
    pub struct ArticleResultPreview {
        pub page_id: i32,
        pub snippet: String,
        pub title: String
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

impl From<wiki::article::ArticleResponse> for wiki::Article {
    fn from(article_response: wiki::article::ArticleResponse) -> Self {
        wiki::Article {
            page_id: article_response.query.pages[0].page_id,
            title: article_response.query.pages[0].title.to_string(),
            content: article_response.query.pages[0].content.to_string()
        }
    }
}
