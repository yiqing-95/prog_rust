#[test]
fn test_split_at() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}fn test_split_at2() {
    let text = "I see the eigenvalue in thine eye"; let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue "); assert_eq!(tail, "in thine eye");
}

#[test]
fn test_zero_tuple(){
    assert_eq!((),{})
}

#[test]
fn comma_at_end(){
    // 只有一个元素时 就必须末尾带逗号已区分括号表达式

    // Rust consistently permits an extra trailing comma every‐ where commas are used:
    // function arguments, arrays, struct and enum definitions, and so on.
    // This may look odd to human readers, but it can make diffs easier to read when entries are added and removed at the end of a list.
    assert_eq!((1,2,),(1,2));
}