mod trait_objects;
mod generics;
mod object_friendly_trait;
mod fully_qualified_method_call;
mod iterators;
mod impl_traits;
mod associated_constants;

use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

#[test]
fn test_say_hello() -> std::io::Result<()> {
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // works
    let mut bytes = vec![];
    say_hello(&mut bytes)?; // also works assert_eq!(bytes, b"hello world\n");
    Ok(())
}

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}