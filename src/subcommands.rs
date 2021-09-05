pub fn cargo_builtin(args: &[&str]) -> std::io::Result<()> {
    use std::process::{exit, Command};

    let status = Command::new("cargo").args(args).status()?;
    exit(status.code().unwrap_or(-1));
}

pub fn audit(force: bool) -> std::io::Result<()> {
    todo!("Implement audit with force option: {:?}", force);

    /*
    use cargo_audit::application::APPLICATION;

    abscissa_core::boot(&APPLICATION);
    */
}
