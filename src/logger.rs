use crate::env;
use log::{Level, LevelFilter};
use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

struct Logger {
    level: LevelFilter,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let logstr = match record.level() {
            Level::Trace => format!("|  {}", record.args()),
            Level::Debug => format!("|  {}", record.args()),
            _ => format!("{}", record.args()),
        };

        println!("{}", logstr);
    }

    fn flush(&self) {}
}

pub fn init() {
    let level = if *env::RIGHTHOOK_VERBOSE {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let logger = LOGGER.get_or_init(|| Logger { level });

    if let Err(err) = log::set_logger(logger).map(|()| log::set_max_level(level)) {
        eprintln!("failed to initialize logger: {}", err);
    }
}
