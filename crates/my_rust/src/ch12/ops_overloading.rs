#[test]
fn test_add() {
    use std::ops::Add;
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}

// 等号的三个要求： 交换性，传递性 自反性（nan ！= nan）
// f32 and f64 are the only types in the standard library that are PartialEq but not Eq.
// Rust’s f32 and f64 are IEEE standard floating-point values. According to that standard, expressions like 0.0/0.0 and others with no appropriate value must produce special not-a-number values, usually referred to as NaN values. The standard further requires that a NaN value be treated as unequal to every other value—including itself
#[test]
fn test_equivalence_relation() {
    assert!(f64::is_nan(0.0 / 0.0));
    assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);

    assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
    assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    // inclusive
    upper: T, // exclusive
}

use std::cmp::{Ordering, PartialOrd};

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_partial_ord() {
    assert!(Interval { lower: 10, upper: 20 } < Interval { lower: 20, upper: 40 });
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
// Overlapping intervals aren't ordered with respect to each other.
    let left = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));
}