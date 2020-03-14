use crate::{invalid_input, invalid_input_error, IOResult};

pub fn change_directory_to_crate_root() -> IOResult<()> {
    let cratedir = locate_crate_dir()?;
    std::env::set_current_dir(cratedir)?;
    Ok(())
}

fn locate_crate_dir() -> IOResult<String> {
    parse(locate_project()?)
}

fn locate_project() -> IOResult<String> {
    use std::io::Write;
    use std::process::Command;

    let r = Command::new("cargo").arg("locate-project").output()?;

    // Write stderr output no matter what:
    std::io::stderr().write_all(&r.stderr[..])?;

    if r.status.success() {
        String::from_utf8(r.stdout)
            .map_err(|e| invalid_input_error("cargo locate project produced non-utf8", e))
    } else {
        invalid_input("cargo locate-project exit status", r.status)
    }
}

fn parse(lpout: String) -> IOResult<String> {
    let prefix = "{\"root\":\"";
    let suffix = "Cargo.toml\"}\n";
    if lpout.starts_with(prefix) && lpout.ends_with(suffix) {
        let (_, x) = lpout.split_at(prefix.len());
        let (y, _) = x.split_at(x.len() - suffix.len());
        Ok(String::from(y))
    } else {
        invalid_input("Could not parse cargo locate-project output", lpout)
    }
}
