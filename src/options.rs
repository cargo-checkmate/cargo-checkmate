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
    /// If no validation is provided, all are run in the order shown by the `list` subcommand.
    Run {
        #[clap(subcommand)]
        phase: Option<Phase>,
    },
    List(List),
    #[clap(subcommand)]
    GitHook(GitHook),
    #[clap(subcommand)]
    Gh(Gh),
    Readme(Readme),
}

/// List the validation steps in order
#[derive(Debug, PartialEq, Eq, clap::Parser)]
pub struct List {}

impl Options {
    pub fn parse_args<I, T>(it: I) -> Options
    where
        I: IntoIterator<Item = T>,
        OsString: From<T>,
    {
        Self::unwrap_parse_result(Self::try_parse_args(it))
    }

    pub fn try_parse_args<I, T>(it: I) -> clap::error::Result<Options>
    where
        I: IntoIterator<Item = T>,
        OsString: From<T>,
    {
        use clap::Parser;

        let args = skip_binary_names(it)?;
        Self::try_parse_from(args)
    }

    pub fn unwrap_parse_result(optsres: clap::error::Result<Options>) -> Options {
        match optsres {
            Ok(opts) => opts,
            Err(e) => e.exit(),
        }
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
            Subcommand::List(x) => x.execute(),
            Subcommand::GitHook(x) => x.execute(),
            Subcommand::Gh(x) => x.execute(),
            Subcommand::Readme(x) => x.execute(),
        }
    }
}

impl Executable for List {
    fn execute(&self) -> anyhow::Result<()> {
        for phase in Phase::list() {
            println!("{phase}");
        }
        Ok(())
    }
}

fn skip_binary_names<I, T>(args: I) -> clap::error::Result<Vec<OsString>>
where
    I: IntoIterator<Item = T>,
    OsString: From<T>,
{
    use clap::error::Error;
    use clap::error::ErrorKind::{self, InvalidSubcommand, MissingSubcommand};
    use std::path::Path;

    fn mk_err(kind: ErrorKind, detail: &str) -> Error {
        Error::raw(
            kind,
            format!(r#"expecting "cargo-checkmate"; found {detail}"#),
        )
    }

    let mut args = args.into_iter().map(OsString::from).peekable();

    let arg = args
        .next()
        .ok_or_else(|| mk_err(MissingSubcommand, "nothing"))?;

    let file_path = Path::new(arg.as_os_str());
    let file_name = file_path
        .file_name()
        .ok_or_else(|| mk_err(InvalidSubcommand, &format!("{arg:?}")))?;
    let extension = file_path.extension().unwrap_or_else(|| "".as_ref());

    if file_name != "cargo-checkmate" && extension != std::env::consts::EXE_EXTENSION {
        return Err(mk_err(InvalidSubcommand, &format!("{arg:?}")));
    }

    if let Some("checkmate") = args.peek().and_then(|s| s.to_str()) {
        args.next();
    }

    Ok(args.collect())
}

#[cfg(test)]
mod tests;
