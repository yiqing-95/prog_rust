#[test]
fn test_size()
{
    // char represents a single Unicode character, as a 32-bit value.

    // Rust uses the char type for single characters in isolation, but uses the UTF-8 encod‐ ing for strings and streams of text. So, a String represents its text as a sequence of UTF-8 bytes,
    // not as an array of characters.
    assert_eq!(std::mem::size_of::<char>(), 4);
}

#[test]
fn test_convert() {
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncated to eight bits, signed
}

#[test]
fn test_convert2() {
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}