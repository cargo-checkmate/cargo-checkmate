mod git;
mod githubci;

use crate::executable::Executable;
use crate::srcbundle::SourceBundle;

/// manage repository hooks.
#[derive(Debug, clap::Subcommand)]
pub enum Hook {
    /// install repository hooks
    Install(HookTypeOption),
    /// uninstall repository hooks
    Uninstall(HookTypeOption),
}

/// hook type option
#[derive(Debug, clap::Parser)]
pub struct HookTypeOption {
    /// Force modifying the hook even if the contents are unrecognized
    #[clap(long)]
    force: bool,

    /// Hook type: all, git, or github-ci
    #[clap(default_value_t)]
    hook_type: HookType,
}

/// hook type
#[derive(Debug, Default, clap::Parser)]
pub enum HookType {
    /// all hooks
    #[default]
    All,
    /// git hooks
    Git,
    /// GitHub CI hooks
    GithubCI,
}

impl Executable for Hook {
    fn execute(&self) -> anyhow::Result<()> {
        use Hook::*;

        let results: Vec<anyhow::Result<()>> = match self {
            Install(HookTypeOption { force, hook_type }) => hook_type
                .source_bundles()?
                .into_iter()
                .map(|sb| sb.install(*force))
                .collect(),
            Uninstall(HookTypeOption { force, hook_type }) => hook_type
                .source_bundles()?
                .into_iter()
                .map(|sb| sb.uninstall(*force))
                .collect(),
        };

        results.into_iter().fold(Ok(()), merge_results)
    }
}

impl HookType {
    fn source_bundles(&self) -> anyhow::Result<Vec<SourceBundle>> {
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

// TODO: clap already knows how to format these, can we reuse that?
impl std::fmt::Display for HookType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use HookType::*;

        match self {
            All => "all",
            Git => "git",
            GithubCI => "github-ci",
        }
        .fmt(f)
    }
}

// TODO: clap already knows how to format these, can we reuse that?
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

fn merge_results(a: anyhow::Result<()>, b: anyhow::Result<()>) -> anyhow::Result<()> {
    match (a, b) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), err) => err,
        (err, Ok(())) => err,
        (Err(a), Err(b)) => Err(anyhow::anyhow!("{:#}\n{:#}", a, b)),
    }
}

#[cfg(test)]
mod tests;
