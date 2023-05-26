use structopt::StructOpt;

#[derive(StructOpt, Debug, Default, Clone)]
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
    /// Override the configured language of wikipedia. The value can be either the language code or
    /// the name of the language in english or the name in its local language
    pub language: Option<String>,

    #[structopt(long = "config-path")]
    /// Print the path for the config file
    pub print_config_path: bool,

    #[structopt(long = "cache-dir")]
    /// Print the cache directory where the log file and crash-reports are written to
    pub print_cache_dir: bool,
}
