use std::path::Path;
use std::time::SystemTime;

pub fn cargo_builtin(args: &[&str]) -> std::io::Result<()> {
    use std::process::{exit, Command};

    let status = Command::new("cargo").args(args).status()?;
    exit(status.code().unwrap_or(-1));
}

pub fn audit(force: bool) -> std::io::Result<()> {
    if force {
        use abscissa_core::application::Application;
        use cargo_audit::application::APPLICATION as AUDIT_APP;

        Application::run(&AUDIT_APP, vec![String::from("audit")]);

        Ok(())
    } else {
        use std::path::PathBuf;
        use std::process::Command;

        let stamp = PathBuf::from("target/audit.timestamp");
        let stamptime = modified(&stamp)?;
        let locktime = modified("Cargo.lock")?;

        if stamptime > locktime {
            println!("Skipped due to recent timestamp: {:?}", stamp);
            Ok(())
        } else {
            let exe = std::env::current_exe()?;
            let status = dbg!(Command::new(exe).arg("audit").arg("--force")).status()?;

            if status.success() {
                // Touch the timestamp path:
                std::fs::File::create(stamp)?;
            }

            std::process::exit(status.code().unwrap());
        }
    }
}

fn modified<P: AsRef<Path>>(p: P) -> std::io::Result<SystemTime> {
    let pref = p.as_ref();
    if pref.exists() {
        pref.metadata()?.modified()
    } else {
        Ok(SystemTime::UNIX_EPOCH)
    }
}
