# Formatted logger

Based on `log` crate, the formatted logger provides a simple way to log messages to the `std stream` (console output)
with a JSON format.
This crate is useful when you e.g. use k8s and gather logs from stderr/stdout for further filtering by JSON key values.

## Features

- allows to log additional data stored within a simplified hashmap format;
- allows to set allowed targets and target to skip for the output.

## Usage

```rust
use formatted_logger::{log_hashmap, HashMapLogData, JsonLogger};
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;

pub fn init_logger() {
    let logger = JsonLogger::new(
        //targets to allow
        Some(vec!["target1".to_string()]),
        //targets to skip
        Some(vec!["target2".to_string()]),
    );
    log::set_boxed_logger(Box::new(logger)).unwrap();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "trace".to_string());
    log::set_max_level(log::LevelFilter::from_str(log_level.as_str()).unwrap());
}

fn main() {
    init_logger();
    let log_data = log_hashmap! {
        "some_key" => "some_value",
        "some_key2" => 3
    };

    debug!(target: "target1", ctxt = log_data; "A log record 1.");
    debug!(target: "target2", ctxt = log_data; "A log record 2.");
    debug!(target: "target3", ctxt = log_data; "A log record 3.");
    //the result will be:
    //{"file":"src/main.rs","level":"DEBUG","line":"29","message":"A log record 1.","module_path":"formatted_logger_try","some_key":"some_value","some_key2":"3","target":"target1"}
}

```

