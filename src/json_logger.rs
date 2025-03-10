use crate::{FormattedLogger, HashMapLogData, KvCollector};
use log::{Log, Metadata, Record};
use serde_json::json;
use std::collections::HashMap;

#[derive(Default)]
pub struct JsonLogger {
    targets_to_allow: Option<Vec<String>>,
    targets_to_skip: Option<Vec<String>>,
}

impl FormattedLogger for JsonLogger {
    fn targets_to_allow(&self) -> &Option<Vec<String>> {
        &self.targets_to_allow
    }

    fn targets_to_skip(&self) -> &Option<Vec<String>> {
        &self.targets_to_skip
    }

    fn do_log(&self, record: &Record) {
        if !self.is_allowed(record.metadata()) {
            return;
        }
        println!("{}", self.gather_message_with_context(record));
    }
}

impl Log for JsonLogger {
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.is_allowed(metadata)
    }

    fn log(&self, record: &Record) {
        self.log_if_allowed(record);
    }

    fn flush(&self) {}
}

impl JsonLogger {
    #[allow(dead_code)]
    pub fn new(
        targets_to_allow: Option<Vec<String>>,
        targets_to_skip: Option<Vec<String>>,
    ) -> Self {
        JsonLogger {
            targets_to_allow,
            targets_to_skip,
        }
    }

    pub fn set_allowed_targets(&mut self, targets_to_allow: Vec<String>) {
        self.targets_to_allow = Some(targets_to_allow);
    }
    pub fn set_skipped_targets(&mut self, targets_to_skip: Vec<String>) {
        self.targets_to_skip = Some(targets_to_skip);
    }

    fn gather_message_with_context(&self, record: &Record) -> serde_json::Value {
        #[allow(unused_mut)]
        let mut log_entry = crate::log_hashmap! {};
        let mut kv_collector = KvCollector {
            map: &mut log_entry,
        };
        let r = record.key_values().visit(&mut kv_collector);

        if let Err(e) = r {
            let message = json!({"logger_error": e.to_string()});
            println!("{}", message);
        }

        log_entry.insert("message", record.args().to_string().as_str());
        log_entry.insert("level", record.level());

        let level: u8 = match record.level() {
            log::Level::Error => 1,
            log::Level::Warn => 2,
            log::Level::Info => 3,
            log::Level::Debug => 4,
            log::Level::Trace => 5,
        };
        log_entry.insert("level_value", level);

        // Add custom fields if any
        if let Some(module_path) = record.module_path() {
            log_entry.insert("module_path", module_path.to_string().as_str());
        }
        if let Some(file) = record.file() {
            log_entry.insert("file", file.to_string().as_str());
        }
        if let Some(line) = record.line() {
            log_entry.insert("line", line.to_string().as_str());
        }

        log_entry.insert("target", record.target().to_string());

        json!(log_entry.orig_hash_map())
    }
}
