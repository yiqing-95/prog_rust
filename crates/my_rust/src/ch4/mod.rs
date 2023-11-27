
mod ownership ;
mod shared_ownership;

use crate::{Command, COMMADNS};


use linkme::distributed_slice;

#[distributed_slice(COMMADNS)]
fn ch1_command() -> Command {
    /* ... */

    Command::new("ch4")
        .usage("  cargo run ch4 ")
        .action(|| {
            // quickreplace::run();
        })
}

