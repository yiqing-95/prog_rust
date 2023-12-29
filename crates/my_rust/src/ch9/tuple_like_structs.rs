
struct Bounds(usize, usize);

#[test]
fn test_construct(){
    let image_bounds = Bounds(1024, 768);

    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}

// newtype
struct Ascii(Vec<u8>);

// unit-like
struct Onesuch;