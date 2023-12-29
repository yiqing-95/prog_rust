pub mod enum_with_data;
mod generic_enums;
mod matches;
mod bin_tree;

use std::cmp::Ordering;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[test]
fn c_style_enum() {
    enum HttpStatus {
        Ok = 200,
        NotModified = 304,
        NotFound = 404,
        // ...
    }
    use std::mem::size_of;
    // assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8

    // Casting a C-style enum to an integer is allowed:
    assert_eq!(HttpStatus::Ok as i32, 200);

    // 只允许单向转换 反向需要手动做   enum_primitive crate 可以帮助我们做
    fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::Ok),
            304 => Some(HttpStatus::NotModified),
            404 => Some(HttpStatus::NotFound),
            // ...
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}


impl TimeUnit {
    /// Return the plural noun for this time unit.
   pub  fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    /// Return the singular noun for this time unit.
    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}