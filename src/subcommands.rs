#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

pub fn cargo_builtin(args: &[&str]) -> std::io::Result<()> {
    use std::process::{exit, Command};

    let status = Command::new("cargo").args(args).status()?;
    exit(status.code().unwrap_or(-1));
}

pub fn audit() -> std::io::Result<()> {
    use abscissa_core::application::Application;
    use cargo_audit::application::{CargoAuditApplication, APPLICATION};

    CargoAuditApplication::run(&APPLICATION, vec![String::from("audit")]);
    Ok(())
}