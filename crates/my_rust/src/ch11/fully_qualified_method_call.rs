#[test]
fn it_works() {
    "hello".to_string();
    str::to_string("hello");

    ToString::to_string("hello");
    <str as ToString>::to_string("hello");
}