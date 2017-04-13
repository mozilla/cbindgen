use std::io;
use std::io::Write;

use log;
use log::*;

pub struct WarnLogger;
pub struct InfoLogger;

impl WarnLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Warn);
            Box::new(WarnLogger)
        })
    }
}
impl log::Log for WarnLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Warn
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            writeln!(io::stderr(),
                   "{}: {}",
                   record.level(),
                   record.args()).unwrap();
        }
    }
}

impl InfoLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(InfoLogger)
        })
    }
}
impl log::Log for InfoLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            writeln!(io::stderr(),
                   "{}: {}",
                   record.level(),
                   record.args()).unwrap();
        }
    }
}
