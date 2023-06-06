#![deny(warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

mod cdcrate;
mod duration;
mod executable;
mod gh;
mod git;
mod githook;
mod options;
mod phase;
mod readme;
mod resultsdir;
mod run;
mod runner;
mod subcommands;

pub use self::run::run;
pub use resultsdir::results_dir;
