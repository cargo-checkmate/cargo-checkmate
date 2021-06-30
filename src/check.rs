use crate::IOResult;
use enum_iterator::IntoEnumIterator;
use std::fmt;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    setting = AppSettings::NoBinaryName,
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Options {
    #[structopt(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, IntoEnumIterator, StructOpt)]
pub enum Subcommand {
    /// Run all checks.
    Everything,

    /// cargo check: syntax + type checking.
    Check,

    /// Use cargo fmt to check if code is correctly formatted.
    Format,

    /// cargo build: build the default target.
    Build,

    /// cargo test: run automated unit tests.
    Test,

    /// cargo doc: generate docs.
    Doc,

    /// cargo audit: check for security advisories across all dependencies.
    Audit,
}

impl Options {
    pub fn parse_args() -> Options {
        let mut it = std::env::args().peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().map(|s| s.as_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        Options::from_iter(it)
    }

    pub fn execute(&self) -> IOResult<()> {
        self.cmd
            .as_ref()
            .unwrap_or(&Subcommand::Everything)
            .execute()
    }
}

impl Subcommand {
    fn execute(&self) -> IOResult<()> {
        use crate::subcommands::{audit, cargo_builtin};
        use Subcommand::*;

        match self {
            Everything => self.execute_everything(),
            Audit => audit(),
            Build => cargo_builtin(&["build"]),
            Check => cargo_builtin(&["check"]),
            Doc => cargo_builtin(&["doc"]),
            Format => cargo_builtin(&["fmt", "--", "--check"]),
            Test => cargo_builtin(&["test"]),
        }
    }

    fn execute_everything(&self) -> IOResult<()> {
        use crate::runner::Runner;

        let mut runner = Runner::new()?;

        println!(
            "\nrunning {} {} phases",
            Subcommand::VARIANT_COUNT - 1,
            crate::CMDNAME
        );

        for check in Subcommand::into_enum_iter() {
            match check {
                Subcommand::Everything => {}
                _ => runner.run_check(&format!("{}", check))?,
            }
        }

        runner.exit()
    }
}

impl fmt::Display for Subcommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
