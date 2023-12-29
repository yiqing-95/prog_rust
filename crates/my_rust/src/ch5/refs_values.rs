use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn show1(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}


#[test]
fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(),
                                                "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    // show(table);
    show(&table);
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[test]
fn deref() {
    let x = 10;
    let r = &x; // &x is a shared reference to x
    assert!(*r == 10); // explicitly dereference r

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
// &mut y is a mutable reference to y
// explicitly dereference m to set y's value
// and to see y's new value
}

#[test]
fn implicitly_dereferences() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    ;
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    // 因为引用经常使用 所以 . 操作符自动解引用了其左手边的变量
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

fn implicitly_borrow() {
    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v
    (&mut v).sort(); // equivalent, but more verbose
}

#[test]
fn assigning_reference() {
    // Assigning a reference to a variable makes that variable point somewhere new:
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;
    if b { r = &y; }
    assert!(*r == 10 || *r == 20);
}

fn references_to_references() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    // The . operator follows as many references as it takes to find its target:
    assert_eq!(rrr.y, 729);
}

#[test]
fn comparison() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    // assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
    assert!(rx == *rrx); // this is okay
}