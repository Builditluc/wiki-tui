pub struct SearchProperties {
    size: bool,
    wordcount: bool,
    timestamp: bool,

    snippet: bool,
    title_snippet: bool,

    redirect_title: bool,
    redirect_snippet: bool,

    section_title: bool,
    section_snippet: bool,

    file_match: bool,
    category_snippet: bool,
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

impl SearchProperties {
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

    build_getter!(size);
    build_getter!(wordcount);
    build_getter!(timestamp);

    build_getter!(snippet);
    build_getter!(title_snippet);

    build_getter!(redirect_title);
    build_getter!(redirect_snippet);

    build_getter!(section_title);
    build_getter!(section_snippet);

    build_getter!(file_match);
    build_getter!(category_snippet);

    pub fn build(&self) -> String {
        let mut query = "&srprop=".to_string();

        macro_rules! build_value {
            ($value: ident, $value_str: expr) => {
                if self.$value {
                    query.push_str($value_str);
                    query.push('|');
                }
            };
        }

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
                .is_file_match()
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
