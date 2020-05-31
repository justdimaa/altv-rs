use log::{Metadata, Record};

pub struct LoggerConfig {
    pub module_log_level: Option<log::LevelFilter>
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => crate::sdk::log::error(format!("{}", record.args()).as_str()),
                log::Level::Warn => crate::sdk::log::warning(format!("{}", record.args()).as_str()),
                log::Level::Info => crate::sdk::log::info(format!("{}", record.args()).as_str()),
                log::Level::Debug => crate::sdk::log::debug(format!("{}", record.args()).as_str()),
                log::Level::Trace => unimplemented!(),
            }
        }
    }

    fn flush(&self) {}
}
