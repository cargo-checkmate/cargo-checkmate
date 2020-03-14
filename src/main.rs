#![deny(warnings)]

mod cdcrate;
mod check;
mod iohelpers;
mod runner;
mod subcommands;

pub use crate::iohelpers::{invalid_input, invalid_input_error, IOResult};

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> IOResult<()> {
    use crate::check::Check;

    crate::cdcrate::change_directory_to_crate_root()?;
    let check = Check::parse_args(std::env::args())?;
    check.execute()
}
