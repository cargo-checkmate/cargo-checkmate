use crate::executable::Executable;
use crate::gh::Gh;
use crate::githook::GitHook;
use crate::phase::Phase;
use crate::readme::Readme;
use std::ffi::OsString;

#[derive(Debug, PartialEq, Eq, clap::Parser)]
#[clap(
    // Accommodate execution as either `cargo-checkmate …` or `cargo checkmate …`
    no_binary_name = true,
    about = env!("CARGO_PKG_DESCRIPTION"),
    version,
)]
pub struct Options {
    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, PartialEq, Eq, clap::Parser)]
pub enum Subcommand {
    /// Run validations
    ///
    /// If no validation is provided, all are run.
    Run {
        #[clap(subcommand)]
        phase: Option<Phase>,
    },
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
        use clap::Parser;

        let mut it = it.into_iter().map(|s| s.into());
        skip_binary_names(&mut it)?;
        Self::try_parse_from(it)
    }
}

impl Executable for Options {
    fn execute(&self) -> anyhow::Result<()> {
        let default = Subcommand::Run { phase: None };
        self.cmd.as_ref().unwrap_or(&default).execute()
    }
}

impl Executable for Subcommand {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Subcommand::Run { phase: x } => x.execute(),
            Subcommand::GitHook(x) => x.execute(),
            Subcommand::Gh(x) => x.execute(),
            Subcommand::Readme(x) => x.execute(),
        }
    }
}

fn skip_binary_names<I>(args: &mut I) -> clap::error::Result<()>
where
    I: Iterator<Item = OsString>,
{
    let bin = expect_arg(&["cargo", "cargo-checkmate"], args)?;

    if bin == "cargo" {
        expect_arg(&["checkmate"], args)?;
    }
    Ok(())
}

fn expect_arg<I>(expecting: &[&str], args: &mut I) -> clap::error::Result<OsString>
where
    I: Iterator<Item = OsString>,
{
    use clap::error::Error;
    use clap::error::ErrorKind::{InvalidSubcommand, MissingSubcommand};
    use std::path::Path;

    let arg = args.next().ok_or_else(|| {
        Error::raw(
            MissingSubcommand,
            format!("expecting one of {expecting:?}; found nothing"),
        )
    })?;

    let file_name = Path::new(arg.as_os_str()).file_name().ok_or_else(|| {
        Error::raw(
            InvalidSubcommand,
            format!("expecting one of {expecting:?}; found {arg:?}"),
        )
    })?;

    for expected in expecting {
        if file_name == *expected {
            return Ok(file_name.to_os_string());
        }
    }

    Err(Error::raw(
        InvalidSubcommand,
        format!("expecting one of {expecting:?}; found {arg:?}"),
    ))
}

#[cfg(test)]
mod tests;
