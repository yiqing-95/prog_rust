pub fn run() {
    main();
}

fn main() {}

#[test]
fn test_constants() {
    // std::f32::consts and std::f64::consts modules provide various commonly used mathematical constants like E, PI
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);
}

#[test]
fn test_mathematical_calculations() {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    // method calls have a higher precedence than prefix operators
    assert_eq!((-1.01f64).floor(), -2.0);
}