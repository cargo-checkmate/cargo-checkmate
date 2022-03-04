use crate::executable::Executable;
use crate::IOResult;
use structopt::StructOpt;

/// manage repository hooks.
#[derive(Debug, StructOpt)]
pub enum Hook {
    /// install repository hooks
    Install(HookTypeOption),
    /// uninstall repository hooks
    Uninstall(HookTypeOption),
}

/// hook type option
#[derive(Debug, StructOpt)]
pub struct HookTypeOption {
    /// Hook type: all, git, or github-ci
    #[structopt(default_value)]
    hook_type: HookType,
}

/// hook type
#[derive(Debug, StructOpt)]
pub enum HookType {
    /// all hooks
    All,
    /// git hooks
    Git,
    /// GitHub CI hooks
    GithubCI,
}

impl Default for HookType {
    fn default() -> HookType {
        HookType::All
    }
}

impl Executable for Hook {
    fn execute(&self) -> IOResult<()> {
        use Hook::*;

        match self {
            Install(HookTypeOption { hook_type }) => todo!("install {:?}", hook_type),
            Uninstall(HookTypeOption { hook_type }) => todo!("uninstall {:?}", hook_type),
        }
    }
}

// TODO: structopt/clap already knows how to format these, can we reuse that?
impl std::fmt::Display for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use HookType::*;

        write!(
            f,
            "{}",
            match self {
                All => "all",
                Git => "git",
                GithubCI => "github-ci",
            }
        )
    }
}

// TODO: structopt/clap already knows how to format these, can we reuse that?
impl std::str::FromStr for HookType {
    type Err = String;

    fn from_str(src: &str) -> Result<HookType, String> {
        use HookType::*;

        if src == "all" {
            Ok(All)
        } else if src == "git" {
            Ok(Git)
        } else if src == "github-ci" {
            Ok(GithubCI)
        } else {
            Err(format!("Unknown hook: {:?}", src))
        }
    }
}
