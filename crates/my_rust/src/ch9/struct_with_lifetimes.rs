struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

// 可以缺省生命周期：omit the lifetimes ：fn find_extrema(slice: &[i32]) -> Extrema {
fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i]; }
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema { greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}