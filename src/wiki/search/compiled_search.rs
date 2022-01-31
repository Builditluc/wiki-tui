use crate::wiki::search::{info::SearchInfo, result::SearchResult};

#[derive(Clone)]
pub struct Search {
    search_offset: usize,
    info: SearchInfo,
    results: Vec<SearchResult>,
}

impl Search {
    pub fn new(search_offset: usize, info: SearchInfo, results: Vec<SearchResult>) -> Self {
        Search {
            search_offset,
            info,
            results,
        }
    }

    pub fn search_offset(&self) -> &usize {
        &self.search_offset
    }
    pub fn info(&self) -> &SearchInfo {
        &self.info
    }
    pub fn results(&self) -> impl Iterator<Item = &SearchResult> {
        self.results.iter()
    }
}
