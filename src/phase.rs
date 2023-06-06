use crate::executable::Executable;

use std::fmt;

#[derive(Debug, PartialEq, Eq, clap::Parser)]
pub enum Phase {
    /// `cargo check` syntax + type checking.
    Check,
    /// Use `cargo fmt` to check if code is correctly formatted.
    Format,
    /// `cargo build` the default target.
    Build,
    /// `cargo test` automated unit tests.
    Test,
    /// `cargo doc` generation.
    Doc,
    /// `cargo clippy` lint checks.
    Clippy,
    /// `cargo audit` security advisories across all dependencies.
    Audit(AuditOptions),
}

#[derive(Debug, PartialEq, Eq, clap::Parser)]
pub struct AuditOptions {
    #[clap(short, long, help = "Force an audit check.")]
    force: bool,
}

impl Phase {
    pub fn execute_everything() -> anyhow::Result<()> {
        use crate::runner::Runner;

        let everything: Vec<Phase> = Phase::list().collect();

        crate::cdcrate::change_directory_to_crate_root()?;
        let mut runner = Runner::new()?;

        println!(
            "\nrunning {} {} validations",
            everything.len(),
            env!("CARGO_PKG_NAME")
        );

        for phase in everything {
            runner.run_phase(&phase)?;
        }

        runner.exit()
    }

    /// List the validation phases executed by [Phase::execute_everything] in order
    pub fn list() -> impl Iterator<Item = Phase> {
        use Phase::*;

        [
            Check,
            Format,
            Clippy,
            Build,
            Test,
            Doc,
            Audit(AuditOptions { force: false }),
        ]
        .into_iter()
    }

    /// The maximum phase name in characters
    pub fn max_phase_name_length() -> usize {
        Phase::list()
            .map(|p| p.to_string().chars().count())
            .max()
            .unwrap()
    }
}

impl Executable for Option<Phase> {
    fn execute(&self) -> anyhow::Result<()> {
        if let Some(phase) = self {
            phase.execute()
        } else {
            Phase::execute_everything()
        }
    }
}

impl Executable for Phase {
    fn execute(&self) -> anyhow::Result<()> {
        use crate::subcommands::{audit, cargo_builtin};
        use Phase::*;

        crate::cdcrate::change_directory_to_crate_root()?;
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
