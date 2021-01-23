-- Your SQL goes here
CREATE TABLE article_index
(
    id         INTEGER   NOT NULL PRIMARY KEY,
    page_id    INTEGER   NOT NULL,
    article_id INTEGER   NOT NULL,
    namespace  INTEGER   NOT NULL,
    title      VARCHAR   NOT NULL,

    updated_at TIMESTAMP NOT NULL
)