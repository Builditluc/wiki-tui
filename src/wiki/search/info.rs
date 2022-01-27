pub struct SearchInfo {
    total_hits: Option<i32>,
    suggestion: Option<String>,
    rewritten_query: Option<String>,
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

impl SearchInfo {
    pub fn new(
        total_hits: Option<i32>,
        suggestion: Option<String>,
        rewritten_query: Option<String>,
    ) -> Self {
        SearchInfo {
            total_hits,
            suggestion,
            rewritten_query,
        }
    }

    build_getter!(total_hits, &i32);
    build_getter!(suggestion, &str);
    build_getter!(rewritten_query, &str);
}
