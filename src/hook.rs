use crate::executable::Executable;
use crate::IOResult;
use structopt::StructOpt;

/// manage repository hooks.
#[derive(Debug, StructOpt)]
pub enum Hook {
    /// install repository hooks
    Install(HookType),
    /// uninstall repository hooks
    Uninstall(HookType),
}

/// manage repository hooks.
#[derive(Debug, StructOpt)]
pub enum HookType {
    /// all hooks
    All,
    /// git hooks
    Git,
    /// GitHub CI hooks
    GithubCI,
}

impl Executable for Hook {
    fn execute(&self) -> IOResult<()> {
        todo!();
    }
}
