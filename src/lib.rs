#![deny(warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

mod cdcrate;
mod executable;
mod git;
mod hook;
mod options;
mod phase;
mod readme;
mod resultsdir;
mod run;
mod runner;
mod srcbundle;
mod subcommands;

pub use self::run::run;
pub use resultsdir::results_dir;

const CMDNAME: &str = env!("CARGO_PKG_NAME");