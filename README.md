# Formatted logger

Based on `log` crate, the formatted logger provides a way to log messages to the `std stream` (console output) together
with the context data presented as hashmap.

## Features

- allows to log additional data stored within a simplified hashmap format;
- allows to set allowed targets and target to skip for the output;
- takes into account the defined log level.

## Variants

- `JsonLogger` - logs the messages in a json format; useful when you e.g. use k8s and gather logs from stderr/stdout for
  further filtering by JSON key values.
- `LineLogger` - logs the messages in a simple line format; useful when you want to have a human-readable log output.

## Usage

```rust
use formatted_logger::{log_hashmap, HashMapLogData, JsonLogger};
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;

pub fn init_logger() {
    let logger = JsonLogger::new(
        //targets to allow
        Some(vec!["target1".to_string(), "target4".to_string()]),
        //targets to skip
        None,
    );
    log::set_boxed_logger(Box::new(logger)).unwrap();
    //log::LOG_LEVEL_NAMES
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
    error!(target: "target4", ctxt = log_data; "Error!");
    error!(target: "target5", ctxt = log_data; "Error2!");

    //for JsonLogger the output will be:
    // {"file":"src/main.rs","level":"DEBUG","level_value":"4","line":"34","message":"A log record 1.","module_path":"formatted_logger_try","some_key":"some_value","some_key2":"3","target":"target1"}
    // {"file":"src/main.rs","level":"ERROR","level_value":"1","line":"37","message":"Error!","module_path":"formatted_logger_try","some_key":"some_value","some_key2":"3","target":"target4"}

    //for LineLogger the output will be:
    // 2025-03-10 12:24:28.547 DEBUG A log record 1. {"some_key":"some_value","some_key2":"3"} 
    // 2025-03-10 12:24:28.548 ERROR Error! {"some_key":"some_value","some_key2":"3"} {"file":"src/main.rs","line":"36","module_path":"formatted_logger_try"}
}

```

