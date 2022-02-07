use crate::executable::Executable;
use crate::githook::GitHook;
use crate::githubci::GithubCI;
use crate::phase::Phase;
use crate::readme::Readme;
use crate::IOResult;
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

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Run all phases.
    Everything,

    #[structopt(flatten)]
    Phase(Phase),

    GitHook(GitHook),
    GithubCI(GithubCI),
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

        Options::from_iter(it)
    }
}

impl Executable for Options {
    fn execute(&self) -> IOResult<()> {
        let default = Subcommand::Everything;
        self.cmd.as_ref().unwrap_or(&default).execute()
    }
}

impl Executable for Subcommand {
    fn execute(&self) -> IOResult<()> {
        match self {
            Subcommand::Everything => Phase::execute_everything(),
            Subcommand::Phase(x) => x.execute(),
            Subcommand::GitHook(x) => x.execute(),
            Subcommand::GithubCI(x) => x.execute(),
            Subcommand::Readme(x) => x.execute(),
        }
    }
}
