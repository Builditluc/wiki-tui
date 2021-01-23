table! {
    article_index (id) {
        id -> Integer,
        page_id -> Integer,
        article_id -> Text,
        namespace -> Integer,
        title -> Text,
        updated_at -> Timestamp,
    }
}
