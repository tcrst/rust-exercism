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
    let logLevel: String = format!("{:?}", level);
    let logMsg: String = format!("[{}]: {}", logLevel.to_uppercase(), message);
    logMsg
    
}
pub fn info(message: &str) -> String {
    let logMsg: String = log(LogLevel::Info, message);
    logMsg 
}
pub fn warn(message: &str) -> String {
    let logMsg: String = log(LogLevel::Warning, message);
    logMsg
}
pub fn error(message: &str) -> String {
   let logMsg: String = log(LogLevel::Error, message);
   logMsg
}
