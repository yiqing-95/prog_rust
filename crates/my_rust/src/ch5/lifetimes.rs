#[derive(Debug)]
struct Config {
    app_name: String,
}

#[derive(Debug)]
struct App<'a> {
    config: &'a Config,
}

// impl App<'a>{
//     pub fn config(&self) -> &Config {
//         self.config
//     }
// }

#[test]
fn test_app() {
    let c = Config { app_name: "my_great_app".into() };
    let a = App {
        config: &c,
    };
    println!("{a:?}");
    println!("{a:#?}");
}

fn ref_args() {
    // This code has several problems, and doesn't compile.
    // static mut STASH: &i32;
    // fn f(p: &i32) { STASH = p; }
}

static mut STASH: &i32 = &128;

// fn f<'a>(p: &'a i32) {
// fn f(p: &i32) { // still not good enough
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

#[test]
fn test_f() {
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}


#[test]
fn test_returning_ref() {
    // v should have at least one element.
    // fn smallest<'a>(v: &'a [i32]) -> &'a i32
    //  When a func‐ tion takes a single reference as an argument and returns a single reference, Rust assumes that the two must have the same lifetime.
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r; }
        }
        s
    }

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0); // fine: parabola still alive
    }
}

#[test]
fn refs_in_struct() {
    // The lifetime of any reference you store in r had better enclose 'a, and 'a must outlast the lifetime of wherever you store the S.
    struct S<'a> {
        r: &'a i32,
    }

    struct D<'a> {
        s: S<'a>,
    }
}

fn distinct_lifetime() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    fn f0<'a>(r: &'a i32, s: &'a i32) -> &'a i32 { r } // perhaps too tight
    fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 { r } // looser
}


#[test]
fn omitting_lifetime_params() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    // shorthand for: fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32
    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }
    // 等价：fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32)
    fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[2])
    }

    struct StringTable {
        elements: Vec<String>,
    }
    impl StringTable {
        // shorthand for: fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }
}


#[test]
fn sharing_vs_mutation() {
    fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
        for elt in slice {
            vec.push(*elt);
        }
    }

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);
// extend wave with another vector
// extend wave with an array
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // extend(&mut wave, &wave); // 不可变借用 跟可变借用同时发生 ；在vec扩容情况下 导致空指针现象
    // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
    //                       0.0, 1.0, 0.0, -1.0]);
}