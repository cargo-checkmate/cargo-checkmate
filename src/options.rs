use crate::executable::Executable;
use crate::gh::Gh;
use crate::githook::GitHook;
use crate::phase::Phase;
use crate::readme::Readme;

#[derive(Debug, clap::Parser)]
#[clap(
    setting = clap::AppSettings::NoBinaryName,
    about = env!("CARGO_PKG_DESCRIPTION"),
    version,
)]
pub struct Options {
    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, clap::Parser)]
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
    pub fn parse_args() -> Options {
        let mut it = std::env::args().peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().map(|s| s.as_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        {
            use clap::Parser;

            Self::from_clap(
                &Self::clap()
                    .bin_name("cargo-checkmate")
                    .get_matches_from(it),
            )
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
