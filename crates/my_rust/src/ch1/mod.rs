pub mod handling_command_line_arguments;
pub mod actix_gcd;
mod mandelbrot;

use crate::{Command, COMMADNS};

// inventory::submit! {
//     // Command::new('v', "verbose")
//     Command::new( "v")
// }

use linkme::distributed_slice;

#[distributed_slice(COMMADNS)]
static BENCH_DESERIALIZE: fn()->Command = ch1_command;

fn ch1_command()->Command {
    /* ... */

    Command::new( "v")
    .action(||{
        println!("some v...") ;
    })
}


#[distributed_slice(COMMADNS)]
fn entry_command()->Command {
    /* ... */

    Command::new( "actix_gcd").action(||{
    //    actix_gcd::run();
    println!("异步运行actix服务器 TODO...");
    actix_gcd::run() ;
    })
}


#[distributed_slice(COMMADNS)]
fn mandelbrot_command()->Command {
    /* ... */

    //  cargo run mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
    Command::new( "mandelbrot")
        .usage(" cargo run mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20")
        .action(||{
        //    actix_gcd::run();
        println!("mandelbrot...>");
        mandelbrot::run() ;
    })
}

