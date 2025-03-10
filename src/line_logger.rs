use crate::HashMapLogData;
use crate::{FormattedLogger, KvCollector};
use log::{Log, Metadata, Record};
use serde_json::json;
use std::collections::HashMap;

#[derive(Default)]
pub struct LineLogger {
    targets_to_allow: Option<Vec<String>>,
    targets_to_skip: Option<Vec<String>>,
}

impl FormattedLogger for LineLogger {
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

        #[allow(unused_mut)]
        let mut context_data = crate::log_hashmap! {};
        let mut kv_collector = KvCollector {
            map: &mut context_data,
        };
        let r = record.key_values().visit(&mut kv_collector);

        if let Err(e) = r {
            println!("Unable to extract log context data: {}", e);
        }

        let path_data_fn = || {
            if record.level() > log::Level::Warn {
                return "".to_string();
            }
            #[allow(unused_mut)]
            let mut path_data = crate::log_hashmap! {};
            if let Some(module_path) = record.module_path() {
                path_data.insert("module_path", module_path.to_string().as_str());
            }
            if let Some(file) = record.file() {
                path_data.insert("file", file.to_string().as_str());
            }
            if let Some(line) = record.line() {
                path_data.insert("line", line.to_string().as_str());
            }

            json!(path_data.orig_hash_map()).to_string()
        };

        println!(
            "{} {} {} {} {}",
            // chrono::Local::now().to_rfc3339(),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level().to_string().as_str(),
            record.args().to_string().as_str(),
            json!(context_data.orig_hash_map()),
            path_data_fn()
        );
    }
}

impl Log for LineLogger {
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.is_allowed(metadata)
    }

    fn log(&self, record: &Record) {
        self.log_if_allowed(record);
    }

    fn flush(&self) {}
}

impl LineLogger {
    #[allow(dead_code)]
    pub fn new(
        targets_to_allow: Option<Vec<String>>,
        targets_to_skip: Option<Vec<String>>,
    ) -> Self {
        LineLogger {
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
}
