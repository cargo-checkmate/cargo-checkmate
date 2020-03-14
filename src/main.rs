#![deny(warnings)]

mod cdcrate;
mod check;
mod iohelpers;
mod phases;
mod runner;
mod subcommands;

pub use crate::iohelpers::{invalid_input, invalid_input_error, IOResult};

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> IOResult<()> {
    use crate::check::Check;

    crate::cdcrate::change_directory_to_crate_root()?;

    match Check::parse_args(std::env::args())? {
        Check::Everything => runner::run(phases::PHASES),
        Check::Audit => subcommands::audit(),
        other => unimplemented!("check {}", other),
    }
}
