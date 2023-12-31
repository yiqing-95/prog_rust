mod quickreplace;

use crate::{Command, COMMADNS};

// inventory::submit! {
//     // Command::new('v', "verbose")
//     Command::new( "v")
// }

use linkme::distributed_slice;

#[distributed_slice(COMMADNS)]
fn ch1_command()->Command {
    /* ... */

    Command::new( "quickreplace")
        .usage("  cargo run quickreplace  \"find\" \"replace\"  Cargo.toml Copy.toml")
        .action(||{
            quickreplace::run();
        })
}

#[test]
fn test() {}
