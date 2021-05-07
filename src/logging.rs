pub struct Logger;
impl Logger {
    pub fn new() {
        use log4rs::append::file::FileAppender;
        use log4rs::config::{Appender, Config, Root};
        use log4rs::encode::pattern::PatternEncoder;

        let wiki_tui = FileAppender::builder()
            .append(false)
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)}] (({I})) [{h({l})}]  {m}\n",
            )))
            .build("wiki_tui.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();
        log4rs::init_config(config).unwrap();
    }
}
