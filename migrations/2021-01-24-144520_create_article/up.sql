-- Your SQL goes here
CREATE TABLE articles
(
    id         INTEGER     NOT NULL PRIMARY KEY,
    article_id VARCHAR(36) NOT NULL,
    title      VARCHAR     NOT NULL,
    text       TEXT        NOT NULL,

    updated_at TIMESTAMP   NOT NULL
)