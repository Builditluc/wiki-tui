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

table! {
    articles (id) {
        id -> Integer,
        article_id -> Text,
        title -> Text,
        text -> Text,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    article_index,
    articles,
);
