mod json_logger;
mod line_logger;

//re-exporting modules structures so crate users can access directly via formatted_logger::StructName
pub use json_logger::JsonLogger;
pub use line_logger::LineLogger;

use log::kv::{Error, Key, ToValue, Value, Visitor};
use log::Metadata;
use std::collections::HashMap;

pub trait FormattedLogger {
    fn targets_to_allow(&self) -> &Option<Vec<String>>;
    fn targets_to_skip(&self) -> &Option<Vec<String>>;

    fn is_allowed(&self, metadata: &Metadata) -> bool {
        if metadata.level() > log::max_level() {
            return false;
        }

        if let Some(targets_to_allow) = &self.targets_to_allow() {
            if !&targets_to_allow.contains(&metadata.target().to_string()) {
                return false;
            }
        }

        if let Some(targets_to_skip) = &self.targets_to_skip() {
            if targets_to_skip.contains(&metadata.target().to_string()) {
                return false;
            }
        }
        true
    }

    fn log_if_allowed(&self, record: &log::Record) {
        if !self.is_allowed(record.metadata()) {
            return;
        }
        self.do_log(record);
    }

    fn do_log(&self, record: &log::Record);
}

#[derive(Clone)]
pub struct HashMapLogData(pub HashMap<String, String>);
impl HashMapLogData {
    pub fn insert<T: ToString, U: ToString>(&mut self, key: T, value: U) {
        self.0.insert(key.to_string(), value.to_string());
    }
    pub fn orig_hash_map(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}

// Custom visitor to collect key-value pairs
struct KvCollector<'a> {
    map: &'a mut HashMapLogData,
}
impl ToValue for HashMapLogData {
    fn to_value(&self) -> Value {
        Value::from_serde(&self.0)
    }
}
impl<'kvs> Visitor<'kvs> for KvCollector<'_> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), Error> {
        if key.as_str() == "ctxt" {
            let json_string = value.to_string();
            let hm: HashMap<String, String> = serde_json::from_str(&json_string).unwrap();
            for (k, v) in hm {
                self.map.insert(k, v);
            }
        } else {
            self.map.insert(key.to_string(), value.to_string());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! log_hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         // let mut map = HashMap::new();
         let mut map = HashMapLogData(HashMap::new());
         $( map.insert($key, $val); )*
         // HashMapLogData(map)
         map
    }}
    }
