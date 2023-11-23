pub mod ch1;
pub mod ch2;

// pub type Action = fn(&Context);
pub type Action = fn();

/// Application command type
#[derive(Default)]
// pub struct Flag {
pub struct Command {
    // short: char,
    name: &'static str,
    /* ... */
    /// Command action
    pub action: Option<Action>,
}

// impl Flag {
impl Command {
    // TODO: 学习const函数的特点 为啥不能使用Default::default
    // pub const  fn new( name: &'static str) -> Self {
    pub fn new(name: &'static str) -> Self {
        Self {
            // short,
            name,
            ..Default::default()
        }
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }
}

use core::panic;
use std::env;

// inventory::collect!(Command);
// 关于linkme的用法 @see https://github.com/dtolnay/linkme
use linkme::distributed_slice;

#[distributed_slice]
// pub static BENCHMARKS: [fn(&mut Bencher)];
// TODO: 可以把通用依赖通过为fn添加Context App之类方法传递进去 fn(Context ...)->Command
pub static COMMADNS: [fn() -> Command];

fn main() {
    println!("Hello, world!");

    // for cmd in inventory::iter::<Command> {
    //     // println!("-{}, --{}", cmd.short, cmd.name);
    //     println!(" cmd name: --{}",   cmd.name);
    // }

    // Iterate the elements.
    //  for cmd_fn in COMMADNS {
    //     /* ... */
    //     let cmd = cmd_fn() ;
    //     println!(" cmd name: --{}",   cmd.name);
    // }

    let _ = _main();
}

fn _main() {
    use std::str::FromStr;

    println!("real main logic here!");

    // https://crates.io/crates/env-settings
    // let env_args = env::args() ;
    let args: Vec<String> = env::args().collect();
    println!("args len: {}", args.len());
    if args.len() < 2 {
        eprintln!("Usage: <some_cmd> ..."); 
        // return ;
        panic!("[error]: need command !");
    }
    // let user_cmd = first(&args).unwrap();
    let user_cmd = args.get(1).unwrap();

    for cmd_fn in COMMADNS {
        //     /* ... */
            let cmd = cmd_fn() ;
              println!(" cmd name: {}",   cmd.name);
              if   user_cmd.eq_ignore_ascii_case(cmd.name){
                println!("OK do it !");

                if cmd.action.is_some() {
                    cmd.action.unwrap()();
                }else {
                    println!("no fn to exec!!! ");
                }

                return ;
              }
    }
    return ;



    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// @see https://stackoverflow.com/questions/36876570/return-first-item-of-vector
fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
