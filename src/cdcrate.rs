use anyhow::Context;

pub fn change_directory_to_crate_root() -> anyhow::Result<()> {
    let cratedir = locate_crate_dir()?;
    std::env::set_current_dir(cratedir)?;
    Ok(())
}

fn locate_crate_dir() -> anyhow::Result<String> {
    parse(locate_project()?)
}

fn locate_project() -> anyhow::Result<String> {
    use anyhow_std::CommandAnyhow;
    use std::io::Write;
    use std::process::Command;

    let r = Command::new("cargo")
        .arg("locate-project")
        .output_anyhow()?;

    // Write stderr output no matter what:
    std::io::stderr().write_all(&r.stderr[..])?;

    if r.status.success() {
        String::from_utf8(r.stdout)
            .with_context(|| "cargo locate project produced non-utf8".to_string())
    } else {
        Err(anyhow::anyhow!("cargo locate-project exit status").context(r.status))
    }
}

fn parse(lpout: String) -> anyhow::Result<String> {
    let prefix = "{\"root\":\"";
    let suffix = "Cargo.toml\"}\n";
    if lpout.starts_with(prefix) && lpout.ends_with(suffix) {
        let (_, x) = lpout.split_at(prefix.len());
        let (y, _) = x.split_at(x.len() - suffix.len());
        Ok(String::from(y))
    } else {
        Err(anyhow::anyhow!("Could not parse cargo locate-project output").context(lpout))
    }
}
