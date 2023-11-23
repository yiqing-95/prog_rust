pub mod ch1;

// pub type Action = fn(&Context);
pub type Action = fn( );

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
    pub const  fn new( name: &'static str) -> Self {
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

inventory::collect!(Command);

fn main() {
    println!("Hello, world!");

    for cmd in inventory::iter::<Command> {
        // println!("-{}, --{}", cmd.short, cmd.name);
        println!(" cmd name: --{}",   cmd.name);
    }
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
