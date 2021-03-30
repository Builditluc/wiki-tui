pub mod wiki {
    pub mod search {
    	use serde::*;

        #[derive(Deserialize, Debug)]
        pub struct SearchResponse {
            #[serde(rename="continue")]
            continue_code: ContinueCode,
            query: QuerySearch
        }

        #[derive(Deserialize, Debug)]
        pub struct ContinueCode {
            #[serde(rename="continue")]
            pub continue_code: String,
            #[serde(rename="sroffset")]
            pub scroll_offset: i32
        }

        #[derive(Deserialize, Debug)]
        pub struct QuerySearch {
            search: Vec<SearchResult>,
            #[serde(rename="searchinfo")]
            search_info: SearchInfo
        }

        #[derive(Deserialize, Debug)]
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

        #[derive(Deserialize, Debug)]
        pub struct SearchInfo {
            #[serde(rename="totalhits")]
            total_hits: i32
        }
    }
    pub mod article {
    	use serde::*;

        #[derive(Deserialize, Debug)]
        pub struct ArticleResponse {
            #[serde(rename="parse")]
            parsed_content: Parse
        }

        #[derive(Deserialize, Debug)]
        pub struct Parse {
            text: ParseText
        }

        #[derive(Deserialize, Debug)]
        pub struct ParseText {
            #[serde(rename="*")]
            content: String
        }
    }
    pub struct ArticleResultPreview {
        pub page_id: i32,
        pub snippet: String,
        pub title: String
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
