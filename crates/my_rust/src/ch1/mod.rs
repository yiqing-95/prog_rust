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

#[test]
fn test() {}
