#![deny(warnings)]

mod cli;
mod phases;
mod runner;
mod subcommands;

#[cfg(test)]
mod tfi;

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    use crate::cli::{parse_args, Command::*};

    match parse_args(std::env::args())? {
        Everything => runner::run(phases::PHASES),
        Audit => subcommands::audit(),
    }
}
