table! {
    article_index (id) {
        id -> Integer,
        page_id -> Integer,
        article_id -> Integer,
        namespace -> Integer,
        title -> Text,
        updated_at -> Timestamp,
    }
}
