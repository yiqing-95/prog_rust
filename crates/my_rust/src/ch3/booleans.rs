// 来源： 比较操作； 其他类型转换 其他语言中!is_empty()  非空值会被认为是true rust中可不是这样哦

#[test]
fn test_convertions(){
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

// short-circuiting logical operators && and ||

#[test]
fn test_size(){
    // Although a bool needs only a single bit to represent it, Rust uses an entire byte for a
    // bool value in memory, so you can create a pointer to it.
    println!("bool size: {}", std::mem::size_of::<bool>());
}