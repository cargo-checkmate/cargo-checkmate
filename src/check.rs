use crate::IOResult;
use enum_iterator::IntoEnumIterator;
use std::fmt;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, IntoEnumIterator, StructOpt)]
#[structopt(setting = AppSettings::NoBinaryName)]
pub enum Check {
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

impl Check {
    pub fn parse_args() -> Check {
        let mut it = std::env::args().peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().map(|s| s.as_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        Check::from_iter(it)
    }

    pub fn execute(&self) -> IOResult<()> {
        use crate::subcommands::{audit, cargo_builtin};

        match self {
            Check::Everything => self.execute_everything(),
            Check::Audit => audit(),
            Check::Build => cargo_builtin(&["build"]),
            Check::Check => cargo_builtin(&["check"]),
            Check::Doc => cargo_builtin(&["doc"]),
            Check::Format => cargo_builtin(&["fmt", "--", "--check"]),
            Check::Test => cargo_builtin(&["test"]),
        }
    }

    fn execute_everything(&self) -> IOResult<()> {
        use crate::runner::Runner;

        let mut runner = Runner::new()?;

        println!(
            "\nrunning {} {} phases",
            Check::VARIANT_COUNT - 1,
            crate::CMDNAME
        );

        for check in Check::into_enum_iter() {
            match check {
                Check::Everything => {}
                _ => runner.run_check(&format!("{}", check))?,
            }
        }

        runner.exit()
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
