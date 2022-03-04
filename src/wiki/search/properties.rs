/// SearchProperties can be used to configure what properties each search result should contain
pub struct SearchProperties {
    /// The size of the article
    size: bool,
    /// The wordcount of the article
    wordcount: bool,
    /// The timestamp of its last edit
    timestamp: bool,

    /// A snippet for the article
    snippet: bool,
    /// A title snippet for the article
    title_snippet: bool,

    /// If it's a redirect, also return the title of the redirect
    redirect_title: bool,
    /// If it's a redirect, also return the snippet of the redirect
    redirect_snippet: bool,

    /// If it's a section, also return the title of the section
    section_title: bool,
    /// If it's a section, also return the snippet of the section
    section_snippet: bool,

    /// If it's a file match
    file_match: bool,
    /// If it's a category, also return the snippet for the category
    category_snippet: bool,
}

/// A helper macro for generating setter functions in the SearchProperties struct
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

impl SearchProperties {
    /// Creates a new SearchProperties struct with its defaults
    pub fn new() -> Self {
        SearchProperties {
            size: false,
            wordcount: false,
            timestamp: false,

            snippet: false,
            title_snippet: false,

            redirect_title: false,
            redirect_snippet: false,

            section_title: false,
            section_snippet: false,

            file_match: false,
            category_snippet: false,
        }
    }

    build_setter!(
        /// The size of the article
        size
    );
    build_setter!(
        /// The wordcount of the article
        wordcount
    );
    build_setter!(
        /// The timestamp of the article
        timestamp
    );

    build_setter!(
        /// A snippet for the article
        snippet
    );
    build_setter!(
        /// A title snippet for the article
        title_snippet
    );

    build_setter!(
        /// If it's a redirect, also return the title of the redirect
        redirect_title
    );
    build_setter!(
        /// If it's a redirect, also return the title of the redirect
        redirect_snippet
    );

    build_setter!(
        /// If it's a section, also return the title of the section
        section_title
    );
    build_setter!(
        /// If it's a section, also return the title of the section
        section_snippet
    );

    build_setter!(
        /// If it's a file match
        file_match
    );
    build_setter!(
        /// If it's a category, also return the snippet for the category
        category_snippet
    );

    /// This function generates a url parameter for itself
    pub fn build(&self) -> String {
        let mut query = "&srprop=".to_string();

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
        build_value!(size, "size");
        build_value!(wordcount, "wordcount");
        build_value!(timestamp, "timestamp");

        build_value!(snippet, "snippet");
        build_value!(title_snippet, "titlesnippet");

        build_value!(redirect_title, "redirecttitle");
        build_value!(redirect_snippet, "redirectsnippet");

        build_value!(section_title, "sectiontitle");
        build_value!(section_snippet, "sectionsnippet");

        build_value!(file_match, "isfilematch");
        build_value!(category_snippet, "categorysnippet");

        // remove any trailing '|' symbols
        if query.ends_with('|') {
            query.pop();
        }
        query
    }
}

impl Default for SearchProperties {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn build() {
        use super::SearchProperties;
        assert_eq!(SearchProperties::new().build(), "&srprop=".to_string());
    }

    #[test]
    fn complete_build() {
        use super::SearchProperties;
        assert_eq!(
            SearchProperties::new()
                .size()
                .wordcount()
                .timestamp()
                .snippet()
                .title_snippet()
                .redirect_title()
                .redirect_snippet()
                .section_title()
                .section_snippet()
                .file_match()
                .category_snippet()
                .build(),
            "&srprop=size|wordcount|timestamp|snippet|titlesnippet|redirecttitle|redirectsnippet|sectiontitle|sectionsnippet|isfilematch|categorysnippet".to_string()
        );
    }

    #[test]
    fn some_build() {
        use super::SearchProperties;
        assert_eq!(
            SearchProperties::new()
                .size()
                .snippet()
                .category_snippet()
                .build(),
            "&srprop=size|snippet|categorysnippet".to_string()
        )
    }
}
