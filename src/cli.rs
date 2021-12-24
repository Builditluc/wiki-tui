use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(
        short = "s",
        long = "search",
        help = "Search for an article at startup with the given query"
    )]
    pub search_query: Option<String>,

    #[structopt(long = "article-id", help = "Open an article with the given id")]
    pub article_id: Option<i32>,
}
