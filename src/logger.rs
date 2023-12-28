//! Simple logger implementation
use crate::time::SimpleSystemTime;
use log::{Level, Log, Metadata, Record};

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{}: {}: {}: {}",
                SimpleSystemTime::now(),
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
