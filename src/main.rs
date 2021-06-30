#![deny(warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

mod cdcrate;
mod check;
mod iohelpers;
mod runner;
mod subcommands;

pub use crate::iohelpers::{invalid_input, invalid_input_error, IOResult};

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> IOResult<()> {
    use crate::check::Options;

    crate::cdcrate::change_directory_to_crate_root()?;
    let opts = Options::parse_args();
    opts.execute()
}
