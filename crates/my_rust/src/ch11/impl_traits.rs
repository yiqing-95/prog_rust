use std::fmt::Display;

fn print<T: Display>(val: T) {
    println!("{}", val);
}

fn print2(val: impl Display) { println!("{}", val);
}

#[test]
fn impl_trait(){
    print::<i32>(42);
}