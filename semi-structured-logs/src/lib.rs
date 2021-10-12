// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    }
}
macro_rules! loglevel_message_func {
    ($func_name:ident, $level: literal) => {
        pub fn $func_name(message: &str) -> String {
            format!("[{}]: {}", $level, message)
        }
    };
}

loglevel_message_func!(info, "INFO");
loglevel_message_func!(warn, "WARNING");
loglevel_message_func!(error, "ERROR");
loglevel_message_func!(debug, "DEBUG");
