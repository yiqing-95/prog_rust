mod tuple_like_structs;
mod impls;
mod simpled_xml;
mod generic_structs;
mod struct_with_lifetimes;
mod deriv_common_tratis;
mod mutability;

use std::num::ParseIntError;

#[test]
fn main() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

/// A rectangle of eight-bit grayscale pixels.
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}


#[test]
fn init_works() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

pub mod api {
    /// A rectangle of eight-bit grayscale pixels.
    pub struct GrayscaleMap {
        pub pixels: Vec<u8>,
        pub size: (usize, usize),
    }
}


// In this game, brooms are monsters. You'll see.
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

// Receive the input Broom by value, taking ownership.
fn chop(b: Broom) -> (Broom, Broom) {
// Initialize `broom1` mostly from `b`, changing only `height`. Since // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
let mut broom1 = Broom { height: b.height / 2, .. b };
// Initialize `broom2` mostly from `broom1`. Since `String` is not // `Copy`, we must clone `name` explicitly.
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };
// Give each fragment a distinct name.
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

#[test]
fn test_create_broom() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey2.health, 100);
}