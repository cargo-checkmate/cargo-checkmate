#![deny(warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

mod cdcrate;
mod executable;
mod git;
mod hook;
mod iohelpers;
mod options;
mod phase;
mod readme;
mod resultsdir;
mod runner;
mod srcbundle;
mod subcommands;

pub use crate::iohelpers::{invalid_input, invalid_input_error, IOResult};
pub use resultsdir::results_dir;

const CMDNAME: &str = env!("CARGO_PKG_NAME");

fn main() -> IOResult<()> {
    use crate::executable::Executable;
    use crate::options::Options;

    crate::cdcrate::change_directory_to_crate_root()?;
    let opts = Options::parse_args();
    opts.execute()
}
