pub mod handling_command_line_arguments;
pub mod actix_gcd;

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

