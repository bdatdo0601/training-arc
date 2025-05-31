use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_logging(debug: bool) {
    let level_filter = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let console_config = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(local)} [{t}] {h({l})} -- {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console_stdout", Box::new(console_config)))
        .build(
            Root::builder()
                .appender("console_stdout")
                .build(level_filter),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
