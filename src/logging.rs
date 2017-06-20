/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io;
use std::io::Write;

use log;
use log::*;

pub struct TraceLogger;
pub struct WarnLogger;
pub struct InfoLogger;

impl TraceLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Trace);
            Box::new(TraceLogger)
        })
    }
}
impl log::Log for TraceLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
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
