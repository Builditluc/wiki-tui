/// SearchMetadata can be used to configure what metadata the search should return
pub struct SearchMetadata {
    /// The number of results found
    total_hits: bool,
    /// A suggestion for the search query
    suggestion: bool,
    /// The rewritten query
    rewritten_query: bool,
}

/// A helper macro for generating setter functions in the SearchMetadata struct
macro_rules! build_setter {
    ($(#[$meta:meta])* $value: ident) => {
        $(#[$meta])*
        #[must_use]
        pub fn $value(mut self) -> Self {
            self.$value = true;
            self
        }
    };
}

impl SearchMetadata {
    /// Creates a new SearchMetadata struct with its defaults
    pub fn new() -> SearchMetadata {
        SearchMetadata {
            total_hits: false,
            suggestion: false,
            rewritten_query: false,
        }
    }

    build_setter!(
        /// The number of results found
        total_hits
    );
    build_setter!(
        /// A suggestion for the search query
        suggestion
    );
    build_setter!(
        /// The rewritten query
        rewritten_query
    );

    /// This function generates a url parameter for itself
    pub fn build(&self) -> String {
        let mut query = "&srinfo=".to_string();

        // a helper macro used to build values
        macro_rules! build_value {
            ($value: ident, $value_str: expr) => {
                if self.$value {
                    query.push_str($value_str);
                    query.push('|');
                }
            };
        }

        // build the values
        build_value!(total_hits, "totalhits");
        build_value!(suggestion, "suggestion");
        build_value!(rewritten_query, "rewrittenquery");

        // remove any trailing '|' symbols
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
