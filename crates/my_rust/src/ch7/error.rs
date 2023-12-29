/*
return Err(JsonError {
message: "expected ']' at end of array".to_string(), line: current_line,
column: current_column
});

使用 thiserror 的  可以省很多事：
use thiserror::Error;
#[derive(Error, Debug)] #[error("{message:} ({line:}, {column})")]
pub struct MyCustomError{...}
 */
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}


use std::fmt;

// Errors should be printable.
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait,
// but the default definitions for the Error methods are fine.
impl std::error::Error for JsonError { }
