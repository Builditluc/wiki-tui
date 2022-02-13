use crate::wiki::search::{info::SearchInfo, result::SearchResult};

/// A finished search containing the results, additional information and the current search offset
/// used for continuing the search
#[derive(Clone)]
pub struct Search {
    /// Use this offset to continue the search
    search_offset: usize,
    /// The metada of the search
    info: SearchInfo,
    /// The results of the search
    results: Vec<SearchResult>,
}

impl Search {
    /// Creates a new Search with a given offset, metadata and resutls
    pub fn new(search_offset: usize, info: SearchInfo, results: Vec<SearchResult>) -> Self {
        Search {
            search_offset,
            info,
            results,
        }
    }

    /// The search offset required for the next search
    pub fn search_offset(&self) -> &usize {
        &self.search_offset
    }

    /// The metadata of the search
    pub fn info(&self) -> &SearchInfo {
        &self.info
    }
    /// The results of the search
    pub fn results(&self) -> impl Iterator<Item = &SearchResult> {
        self.results.iter()
    }
}
