mod floating_point_types;
mod booleans;
mod characters;
mod tuples;
mod pointer_types;
mod sequence_values;
mod strings;

use crate::{Command, COMMADNS};


use linkme::distributed_slice;

#[distributed_slice(COMMADNS)]
fn ch1_command() -> Command {
    /* ... */

    Command::new("ch3")
        .usage("  cargo run quickreplace  \"find\" \"replace\"  Cargo.toml Copy.toml")
        .action(|| {
            // quickreplace::run();
        })
}


fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

#[test]
fn test_conversions() {
    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(2525_u16 as i16, 2525_i16); // in range
    assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
// Conversions that are out of range for the destination
// produce values that are equivalent to the original modulo 2^N, // where N is the width of the destination in bits. This
// is sometimes called "truncation."
//     assert_eq!(1000_i16asu8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16); // exponentiation
    assert_eq!((-4_i32).abs(), 4); // absolute value
    assert_eq!(0b101101_u8.count_ones(), 4); // population count
}

fn ambiguous_numeric_type() {
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}

//
// In a release build, this multiplication wraps to a negative number, and the loop runs indefinitely.
fn panics_in_debug_build() {
    let mut i = 1;
    loop {
        i *= 10; // panic: attempt to multiply with overflow // (but only in debug builds!)
    }
}

fn panics_in_any_build() {
    let mut i: i32 = 1;
    loop {
        // panic: multiplication overflowed (in any build)
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

#[test]
fn checked_ops() {
    // The sum of 10 and 20 can be represented as a u8.
    assert_eq!(10_u8.checked_add(20), Some(30));
// Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None); // Do the addition; panic if it overflows.
    // let sum = x.checked_add(y).unwrap();
// Oddly, signed division can overflow too, in one particular case. // A signed n-bit type can represent -2n−1, but not 2n−1.
    assert_eq!((-128_i8).checked_div(-1), None);
}

#[test]
fn wrapping_operations() {

// The first product can be represented as a u16; // the second cannot, so we get 250000 modulo 216. assert_eq!(100_u16.wrapping_mul(200), 20000); assert_eq!(500_u16.wrapping_mul(500), 53392);
// Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);
// In bitwise shift operations, the shift distance // is wrapped to fall within the size of the value. // So a shift of 17 bits in a 16-bit type is a shift // of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);
}

#[test]
fn saturating_operations() {
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

#[test]
fn overflowing_operations() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}