pub struct SearchMetadata {
    total_hits: bool,
    suggestion: bool,
    rewritten_query: bool,
}

macro_rules! build_getter {
    ($value: ident) => {
        #[must_use]
        pub fn $value(mut self) -> Self {
            self.$value = true;
            self
        }
    };
}

impl SearchMetadata {
    pub fn new() -> SearchMetadata {
        SearchMetadata {
            total_hits: false,
            suggestion: false,
            rewritten_query: false,
        }
    }

    build_getter!(total_hits);
    build_getter!(suggestion);
    build_getter!(rewritten_query);

    pub fn build(&self) -> String {
        let mut query = "&srinfo=".to_string();

        macro_rules! build_value {
            ($value: ident, $value_str: expr) => {
                if self.$value {
                    query.push_str($value_str);
                    query.push('|');
                }
            };
        }

        build_value!(total_hits, "totalhits");
        build_value!(suggestion, "suggestion");
        build_value!(rewritten_query, "rewrittenquery");

        if query.ends_with('|') {
            query.pop();
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
        assert_eq!(SearchMetadata::new().build(), "&srinfo=".to_string());
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
            "&srinfo=totalhits|rewrittenquery".to_string()
        );
    }
}
