// TODO: add this as a configuration option

/// SearchSortOrder can be used to configure how the search results should be ordered. The default
/// one is relevance
pub enum SearchSortOrder {
    CreateTimestampAscending,
    CreateTimestampDescending,

    IncomingLinksAscending,
    IncomingLinksDescending,

    LastEditAscending,
    LastEditDescending,

    JustMatch,
    NoSort,
    Random,
    Relevance,
    UserRandom,
}

impl Default for SearchSortOrder {
    fn default() -> Self {
        SearchSortOrder::Relevance
    }
}

impl std::fmt::Display for SearchSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchSortOrder::CreateTimestampAscending => write!(f, "&srsort=create_timestamp_asc"),
            SearchSortOrder::CreateTimestampDescending => {
                write!(f, "&srsort=create_timestamp_desc")
            }

            SearchSortOrder::IncomingLinksAscending => write!(f, "&srsort=incoming_links_asc"),
            SearchSortOrder::IncomingLinksDescending => write!(f, "&srsort=incoming_links_desc"),

            SearchSortOrder::LastEditAscending => write!(f, "&srsort=last_edit_asc"),
            SearchSortOrder::LastEditDescending => write!(f, "&srsort=last_edit_desc"),

            SearchSortOrder::JustMatch => write!(f, "&srsort=just_match"),
            SearchSortOrder::NoSort => write!(f, "&srsort=none"),
            SearchSortOrder::Random => write!(f, "&srsort=random"),
            SearchSortOrder::Relevance => write!(f, "&srsort=relevance"),
            SearchSortOrder::UserRandom => write!(f, "&srsort=user_random"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn format() {
        use super::SearchSortOrder;

        assert_eq!(
            SearchSortOrder::CreateTimestampAscending.to_string(),
            "&srsort=create_timestamp_asc".to_string()
        );
        assert_eq!(
            SearchSortOrder::CreateTimestampDescending.to_string(),
            "&srsort=create_timestamp_desc".to_string()
        );

        assert_eq!(
            SearchSortOrder::IncomingLinksAscending.to_string(),
            "&srsort=incoming_links_asc".to_string()
        );
        assert_eq!(
            SearchSortOrder::IncomingLinksDescending.to_string(),
            "&srsort=incoming_links_desc".to_string()
        );

        assert_eq!(
            SearchSortOrder::LastEditAscending.to_string(),
            "&srsort=last_edit_asc".to_string()
        );
        assert_eq!(
            SearchSortOrder::LastEditDescending.to_string(),
            "&srsort=last_edit_desc".to_string()
        );

        assert_eq!(
            SearchSortOrder::JustMatch.to_string(),
            "&srsort=just_match".to_string()
        );
        assert_eq!(
            SearchSortOrder::NoSort.to_string(),
            "&srsort=none".to_string()
        );
        assert_eq!(
            SearchSortOrder::Random.to_string(),
            "&srsort=random".to_string()
        );
        assert_eq!(
            SearchSortOrder::UserRandom.to_string(),
            "&srsort=user_random".to_string()
        );
    }
}
