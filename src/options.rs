use crate::executable::Executable;
use crate::gh::Gh;
use crate::githook::GitHook;
use crate::phase::Phase;
use crate::readme::Readme;
use std::ffi::OsString;

#[derive(Debug, PartialEq, Eq, clap::Parser)]
#[clap(
    setting = clap::AppSettings::NoBinaryName,
    about = env!("CARGO_PKG_DESCRIPTION"),
    version,
)]
pub struct Options {
    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, PartialEq, Eq, clap::Parser)]
pub enum Subcommand {
    /// Run all phases.
    Everything,

    #[clap(flatten)]
    Phase(Phase),

    #[clap(subcommand)]
    GitHook(GitHook),
    #[clap(subcommand)]
    Gh(Gh),
    Readme(Readme),
}

impl Options {
    pub fn parse_std_args() -> Options {
        match Self::try_parse_args(std::env::args()) {
            Ok(opts) => opts,
            Err(e) => e.exit(),
        }
    }

    pub fn try_parse_args<I, T>(it: I) -> clap::error::Result<Options>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut it = it.into_iter().map(|s| s.into()).peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().and_then(|s| s.to_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        {
            use clap::Parser;

            let matches = &Self::clap()
                .bin_name("cargo-checkmate")
                .try_get_matches_from(it)?;

            Ok(Self::from_clap(matches))
        }
    }
}

impl Executable for Options {
    fn execute(&self) -> anyhow::Result<()> {
        let default = Subcommand::Everything;
        self.cmd.as_ref().unwrap_or(&default).execute()
    }
}

impl Executable for Subcommand {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Subcommand::Everything => Phase::execute_everything(),
            Subcommand::Phase(x) => x.execute(),
            Subcommand::GitHook(x) => x.execute(),
            Subcommand::Gh(x) => x.execute(),
            Subcommand::Readme(x) => x.execute(),
        }
    }
}

#[cfg(test)]
mod tests;
