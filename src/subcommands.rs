use anyhow_std::CommandAnyhow;
use std::process::Command;

pub fn cargo_builtin<const K: usize, const L: usize>(
    args: [&str; K],
    envs: [(&str, &str); L],
) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .envs(envs)
        .status_anyhow()?;
    status.exit();
}
