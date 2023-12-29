use crate::ch10::TimeUnit;

/// A timestamp that has been deliberately rounded off, so our program /// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[test]
fn test_new() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
}

#[test]
fn struct_variants() {
    // enum Shape {
    //     Sphere { center: Point3d, radius: f32 },
    //     Cuboid { corner1: Point3d, corner2: Point3d },
    // }
    // let unit_sphere = Shape::Sphere {
    //     center: ORIGIN,
    //     radius: 1.0,
    // };
}

// All constructors and fields of an enum share the same visibility as the enum itself.
// 👇的enum 是三种不同类型的enum综合体
// enum RelationshipStatus { Single,
//     InARelationship,
//     ItsComplicated(Option<String>),
//     ItsExtremelyComplicated {
//         car: DifferentialEquation,
//         cdr: EarlyModernistPoem, },
// }


use std::collections::HashMap;
enum Json { Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>), Object(Box<HashMap<String, Json>>),
}
#[test]
fn test_size_of_json(){
    use std::mem::size_of;
    assert_eq!(size_of::<Json>(), 32);
}