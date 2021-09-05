use crate::executable::Executable;
use crate::IOResult;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Check {
    /// check: `cargo check` syntax + type checking.
    Check,

    /// check: Use `cargo fmt` to check if code is correctly formatted.
    Format,

    /// check: `cargo build` the default target.
    Build,

    /// check: `cargo test` automated unit tests.
    Test,

    /// check: `cargo doc` generation.
    Doc,

    /// check: `cargo audit` security advisories across all dependencies.
    Audit(AuditOptions),
}

#[derive(Debug, StructOpt)]
pub struct AuditOptions {
    #[structopt(short, long, help = "Force an audit check.")]
    force: bool,
}

impl Check {
    pub fn execute_everything() -> IOResult<()> {
        use crate::runner::Runner;

        let everything = &[
            Check::Check,
            Check::Format,
            Check::Build,
            Check::Test,
            Check::Doc,
            Check::Audit(AuditOptions { force: false }),
        ];

        let mut runner = Runner::new()?;

        println!("\nrunning {} {} phases", everything.len(), crate::CMDNAME);

        for check in everything {
            runner.run_check(&format!("{}", check))?;
        }

        runner.exit()
    }
}

impl Executable for Check {
    fn execute(&self) -> IOResult<()> {
        use crate::subcommands::{audit, cargo_builtin};

        match self {
            Check::Audit(opts) => audit(opts.force),
            Check::Build => cargo_builtin(&["build"]),
            Check::Check => cargo_builtin(&["check"]),
            Check::Doc => cargo_builtin(&["doc"]),
            Check::Format => cargo_builtin(&["fmt", "--", "--check"]),
            Check::Test => cargo_builtin(&["test"]),
        }
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
