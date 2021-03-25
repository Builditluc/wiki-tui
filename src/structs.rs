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
            page_id: i32,
            size: i32,
            snippet: String,
            timestamp: String,
            title: String,
            #[serde(rename="wordcount")]
            word_count: i32
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
}
