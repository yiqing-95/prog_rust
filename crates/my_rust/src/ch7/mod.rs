mod panics;
mod results;
mod error;

use std::error::Error;
use std::io::{Write, stderr};

///
// let io_error = io::Error::new( // make our own
// io::Error io::ErrorKind::Other, "timed out");
// return Err(GenericError::from(io_error)); // manually convert to GenericError
// 如果想处理特定类型错误：
// error.downcast_ref::<ErrorType>().
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

type GenericResult<T> = Result<T, GenericError>;


fn real_main() -> Result<(), GenericError> {
    // let tides = calculate_tides()?; print_tides(tides);
    Ok(())
}
fn main() {
    if let Err(err) = real_main() {
        // print_error(err.source().unwrap().deref());
        std::process::exit(1);
    }
}
#[test]
fn test_main(){
    main();
}

use std::io::{BufRead};

/// Read integers from a text file.
/// The file should have one number on each line.
// fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    // The writeln! macro works like println!, except that it writes the data to a stream of your choice.
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

use std::fs;
use std::io;
use std::ops::Deref;
use std::path::Path;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? { // opening dir could fail
        let entry = entry_result?; // reading dir could fail
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // renaming could fail
    }
    Ok(()) // phew!
}