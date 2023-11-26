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
    /// Command usage
    pub usage: Option<String>,
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

    /// Set usage of the command
    ///
    /// Example
    ///
    /// ```
    /// let command = Command::new("cmd")
    ///     .usage("cli cmd [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }
}

use core::panic;
use std::env;

// inventory::collect!(Command);
// 关于linkme的用法 @see https://github.com/dtolnay/linkme
// TODO: 有空了可以看下这个库  ctor 
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
            //   println!(" cmd name: {}",   cmd.name);
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



}

// @see https://stackoverflow.com/questions/36876570/return-first-item-of-vector
fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}


#[distributed_slice(COMMADNS)]
fn help_command()->Command {
    /* ... */

    Command::new( "help")
    .action(||{
        println!("all available commands:") ;
        for cmd_factory in COMMADNS {
            //     /* ... */
                let cmd = cmd_factory() ;
                println!(" ——: {}",   cmd.name);
                if cmd.usage.is_some() {
                    println!("   usage: {}",   cmd.usage.unwrap());
                }
    
                
        }
    })
}