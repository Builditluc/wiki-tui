use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
pub struct Cli {
    /// Search for an article at startup with the given query
    pub search_query: Option<String>,

    #[structopt(long = "article-id")]
    /// Open an article with the given id
    pub article_id: Option<i32>,

    #[structopt(long = "level")]
    /// Override the log level. Levels are:
    /// - Debug: 0
    /// - Info: 1
    /// - Warn: 2
    /// - Error: 3
    pub level: Option<i32>,

    #[structopt(short = "l", long = "language")]
    /// Override the configured language of wikipeida. The value can be either the language code or
    /// the name of the language in english or the name in its local language
    pub language: Option<String>,
}
