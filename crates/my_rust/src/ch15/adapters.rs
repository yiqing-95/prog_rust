#[test]
fn test_map() {
    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
}

#[test]
fn test_filter() {
    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    // A chain of iterator adapters is like a pipeline in the Unix shell: each adapter has a single purpose,
    // and it’s clear how the sequence is being transformed as one reads from left to right.
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas").collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}