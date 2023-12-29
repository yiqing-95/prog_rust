mod refs_values;
mod lifetimes;

use crate::{Command, COMMADNS};


use linkme::distributed_slice;

#[distributed_slice(COMMADNS)]
fn chapter_command() -> Command {
    /* ... */

    Command::new("ch5")
        .usage("  cargo run ch5 ")
        .action(|| {
            // quickreplace::run();
        })
}

