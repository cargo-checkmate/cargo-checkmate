mod git;
mod githubci;

use crate::executable::Executable;
use crate::srcbundle::SourceBundle;
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

        let results: Vec<IOResult<()>> = match self {
            Install(HookTypeOption { hook_type }) => hook_type
                .source_bundles()?
                .into_iter()
                .map(|sb| sb.install())
                .collect(),
            Uninstall(HookTypeOption { hook_type }) => hook_type
                .source_bundles()?
                .into_iter()
                .map(|sb| sb.uninstall())
                .collect(),
        };

        results.into_iter().fold(Ok(()), merge_std_errs)
    }
}

impl HookType {
    fn source_bundles(&self) -> IOResult<Vec<SourceBundle>> {
        use HookType::*;

        Ok(match self {
            All => vec![
                self::git::source_bundle()?,
                self::githubci::source_bundle()?,
            ],
            Git => vec![self::git::source_bundle()?],
            GithubCI => vec![self::githubci::source_bundle()?],
        })
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

fn merge_std_errs(a: IOResult<()>, b: IOResult<()>) -> IOResult<()> {
    use std::io::{Error, ErrorKind::Other};

    match (a, b) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), err) => err,
        (err, Ok(())) => err,
        (Err(a), Err(b)) => Err(Error::new(Other, format!("{}\n{}", a, b))),
    }
}
