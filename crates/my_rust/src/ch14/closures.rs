struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f32,
    // ...
}
// fn sort_cities(cities: &mut Vec<City>) {
//     cities.sort(); // error: how do you want them sorted?
// }

/// Helper function for sorting cities by population.
fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities0(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending); // ok
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

#[test]
fn fn_type() {
    type MyKeyFn = fn(&City) -> i64;

    let my_key_fn: MyKeyFn = city_population_descending;

    let my_int: isize = 1;
    // assert_eq!(std::mem::size_of_val(&my_key_fn),std::mem::size_of_val(&1));
    assert_eq!(std::mem::size_of_val(&my_key_fn), std::mem::size_of_val(&my_int));
}

/// Given a list of cities and a test function,
/// return how many cities pass the test.
///
/// 函数指针版本 ；能当函数参数的就可以作为某个Struct的成员变量 反之亦然
///
/// trait Fn(&City) -> bool. This trait is automatically implemented by all functions and most closures that take a single &City as an argument and return a Boolean value:
//  fn(&City) -> bool // fn type (functions only)
//  Fn(&City) -> bool // Fn trait (both functions and closures)
fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

#[test]
fn test_count_selected_cities() {
    /// An example of a test function. Note that the type of
    /// this function is `fn(&City) -> bool`, the same as
    /// the `test_fn` argument to `count_selected_cities`.
// How many cities are at risk for monster attack?
// let n = count_selected_cities(&my_cities, has_monster_attacks);
    assert_eq!(1, 1);
}

//  闭包版本
// Since every closure has its own type, code that works with closures usually needs to be generic,
fn count_selected_cities2<F>(cities: &Vec<City>, test_fn: F) -> usize where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

#[test]
fn test_count_selected_cities2() {
    let my_cities = vec![
        City {
            name: "xian".into(),
            population: 1000000,
            country: "china".to_string(),
            monster_attack_risk: 0.1,
        }, City {
            name: "beijing".into(),
            population: 10000000,
            country: "china".to_string(),
            monster_attack_risk: 0.5,
        },
    ];

    let p = count_selected_cities2(&my_cities,
                                   has_monster_attacks); // ok
    assert_eq!(p, 2);


    let limit = 0.2_f32;
    let p = count_selected_cities2(&my_cities,
                                   |city| city.monster_attack_risk > limit); // also ok

    assert_eq!(p, 1);
}

// Fn() is shorthand for Fn() -> ()
fn call_twice<F>(closure: F) where F: Fn() {
    closure();
    closure();
}

#[test]
fn test_call_twice() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);
    // call_twice(f); // 编译器识别f为 FnOnce 而非 Fn

    let mut i = 0;
    // 识别为FnMut
    let incr = || {
        i += 1; // incr borrows a mut reference to i
        println!("Ding! i is now: {}", i);
    };
    // call_twice(incr);
}

// 该版本可以同时接受 Fn 和 FnMut了
fn call_twice2<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}


#[test]
fn test_call_twice2() {
    let mut i = 0;
    call_twice2(|| i += 1); // ok!
    assert_eq!(i, 2);
}

#[test]
fn test_copy_clone() {
    let y = 10;
    let add_y = |x| x + y;

    // let mut x=0;
    // Mutable references are neither Clone nor Copy, so neither is a closure that uses them
    // let mut add_to_x=|n|{x+=n;x};

    // = 此步行为视闭包类型而定 copy 还是 move
    let copy_of_add_y = add_y; // This closure is `Copy`, so...
    assert_eq!(add_y(copy_of_add_y(22i32)), 42); // ... we can call both.
}

#[test]
fn test_move_closure() {
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}