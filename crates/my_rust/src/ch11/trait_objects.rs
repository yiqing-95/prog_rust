use std::io::Write;

#[test]
fn it_works(){
    let mut buf: Vec<u8> = vec![];
    // A reference to a trait type, like writer, is called a trait object
    let writer: &mut dyn Write = &mut buf; // ok

    // let w: Box<dyn Write> = Box::new(local_file);
}