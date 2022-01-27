pub struct SearchResult {
    title: String,
    namespace: usize,
    page_id: i32,
    size: Option<i32>,

    wordcount: Option<i32>,
    timestamp: Option<String>,

    snippet: Option<String>,
    title_snippet: Option<String>,
    category_snippet: Option<String>,

    redirect_title: Option<String>,
    redirect_snippet: Option<String>,

    section_title: Option<String>,
    section_snippet: Option<String>,

    is_file_match: Option<bool>,
}

macro_rules! build_getter {
    ($value: ident, $return_type: ty) => {
        pub fn $value(&self) -> Option<$return_type> {
            match &self.$value {
                Some(ref value) => Some(value),
                None => None,
            }
        }
    };
}

impl SearchResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        namespace: usize,
        page_id: i32,
        size: Option<i32>,
        wordcount: Option<i32>,
        timestamp: Option<String>,
        snippet: Option<String>,
        title_snippet: Option<String>,
        category_snippet: Option<String>,
        redirect_title: Option<String>,
        redirect_snippet: Option<String>,
        section_title: Option<String>,
        section_snippet: Option<String>,
        is_file_match: Option<bool>,
    ) -> Self {
        Self {
            title,
            namespace,
            page_id,
            size,
            wordcount,
            timestamp,
            snippet,
            title_snippet,
            category_snippet,
            redirect_title,
            redirect_snippet,
            section_title,
            section_snippet,
            is_file_match,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn namespace(&self) -> &usize {
        &self.namespace
    }
    pub fn page_id(&self) -> &i32 {
        &self.page_id
    }

    build_getter!(size, &i32);
    build_getter!(wordcount, &i32);
    build_getter!(timestamp, &str);

    build_getter!(snippet, &str);
    build_getter!(title_snippet, &str);
    build_getter!(category_snippet, &str);

    build_getter!(redirect_title, &str);
    build_getter!(redirect_snippet, &str);

    build_getter!(section_title, &str);
    build_getter!(section_snippet, &str);

    pub fn is_file_match(&self) -> Option<&bool> {
        self.is_file_match.as_ref()
    }
}
