#![deny(warnings)]

mod cdcrate;
mod cli;
mod iohelpers;
mod phases;
mod runner;
mod subcommands;

pub use crate::iohelpers::{invalid_input, invalid_input_error, IOResult};

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> IOResult<()> {
    use crate::cli::{parse_args, Command::*};

    crate::cdcrate::change_directory_to_crate_root()?;

    match parse_args(std::env::args())? {
        Everything => runner::run(phases::PHASES),
        Audit => subcommands::audit(),
    }
}
