pub struct SearchMetadata {
    total_hits: bool,
    suggestion: bool,
    rewritten_query: bool,
}

impl SearchMetadata {
    pub fn new() -> SearchMetadata {
        SearchMetadata {
            total_hits: false,
            suggestion: false,
            rewritten_query: false,
        }
    }

    #[must_use]
    pub fn total_hits(mut self) -> Self {
        self.total_hits = true;

        self
    }

    #[must_use]
    pub fn suggestion(mut self) -> Self {
        self.suggestion = true;

        self
    }

    #[must_use]
    pub fn rewritten_query(mut self) -> Self {
        self.rewritten_query = true;

        self
    }

    pub fn build(&self) -> String {
        let mut query = "&srinfo=".to_string();

        if self.total_hits {
            query.push_str("totalhits");
        }

        query.push('|');

        if self.suggestion {
            query.push_str("suggestion");
        }

        query.push('|');

        if self.rewritten_query {
            query.push_str("rewrittenquery");
        }

        query
    }
}

impl Default for SearchMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn build() {
        use super::SearchMetadata;
        assert_eq!(SearchMetadata::new().build(), "&srinfo=||".to_string());
    }

    #[test]
    fn complete_build() {
        use super::SearchMetadata;
        assert_eq!(
            SearchMetadata::new()
                .total_hits()
                .rewritten_query()
                .suggestion()
                .build(),
            "&srinfo=totalhits|suggestion|rewrittenquery".to_string()
        );
    }

    #[test]
    fn some_build() {
        use super::SearchMetadata;
        assert_eq!(
            SearchMetadata::new().total_hits().rewritten_query().build(),
            "&srinfo=totalhits||rewrittenquery".to_string()
        );
    }
}
