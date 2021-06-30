use crate::executable::Executable;
use crate::IOResult;
use structopt::StructOpt;

/// git-hook support.
#[derive(Debug, StructOpt)]
pub enum GitHook {
    /// install git-hook.
    Install,
    Run(RunArgs),
}

/// run git-hook.
#[derive(Debug, StructOpt)]
pub enum RunArgs {
    #[structopt(external_subcommand)]
    RA(Vec<String>),
}

impl Executable for GitHook {
    fn execute(&self) -> IOResult<()> {
        use GitHook::*;

        match self {
            Install => unimplemented!("git-hook install"),
            Run(RunArgs::RA(hookargs)) => unimplemented!("git-hook run {:?}", hookargs),
        }
    }
}
