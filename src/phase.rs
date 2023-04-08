use crate::executable::Executable;

use std::fmt;

#[derive(Debug, clap::Parser)]
pub enum Phase {
    /// phase: `cargo check` syntax + type checking.
    Check,

    /// phase: Use `cargo fmt` to check if code is correctly formatted.
    Format,

    /// phase: `cargo build` the default target.
    Build,

    /// phase: `cargo test` automated unit tests.
    Test,

    /// phase: `cargo doc` generation.
    Doc,

    /// phase: `cargo clippy` lint checks.
    Clippy,

    /// phase: `cargo audit` security advisories across all dependencies.
    Audit(AuditOptions),
}

#[derive(Debug, clap::Parser)]
pub struct AuditOptions {
    #[clap(short, long, help = "Force an audit check.")]
    force: bool,
}

impl Phase {
    pub fn execute_everything() -> std::io::Result<()> {
        use crate::runner::Runner;
        use Phase::*;

        let everything = &[
            Check,
            Format,
            Clippy,
            Build,
            Test,
            Doc,
            Audit(AuditOptions { force: false }),
        ];

        let mut runner = Runner::new()?;

        println!("\nrunning {} {} phases", everything.len(), crate::CMDNAME);

        for phase in everything {
            runner.run_phase(phase)?;
        }

        runner.exit()
    }
}

impl Executable for Phase {
    fn execute(&self) -> std::io::Result<()> {
        use crate::subcommands::{audit, cargo_builtin};
        use Phase::*;

        match self {
            Audit(opts) => audit(opts.force),
            Build => cargo_builtin(&["build"]),
            Check => cargo_builtin(&["check"]),
            Doc => cargo_builtin(&["doc"]),
            Clippy => cargo_builtin(&["clippy"]),
            Format => cargo_builtin(&["fmt", "--", "--check"]),
            Test => cargo_builtin(&["test"]),
        }
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Phase::Audit(opts) => format!("audit{}", if opts.force { " (force)" } else { "" }),
                _ => format!("{:?}", self).to_lowercase(),
            }
        )
    }
}
