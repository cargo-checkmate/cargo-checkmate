use std::path::Path;
use std::time::{Duration, SystemTime};

const AUDIT_EXPIRATION_TIMEOUT_SECS: u64 = 60 * 60 * 3;

pub fn cargo_builtin(args: &[&str]) -> std::io::Result<()> {
    use std::process::{exit, Command};

    let status = Command::new("cargo").args(args).status()?;
    exit(status.code().unwrap_or(-1));
}

pub fn audit(force: bool) -> std::io::Result<()> {
    if force {
        force_audit()
    } else {
        audit_if_necessary()
    }
}

fn force_audit() -> std::io::Result<()> {
    use abscissa_core::application::Application;
    use cargo_audit::application::APPLICATION as AUDIT_APP;

    Application::run(&AUDIT_APP, vec![String::from("audit")]);

    Ok(())
}

fn audit_if_necessary() -> std::io::Result<()> {
    use std::path::PathBuf;
    use std::process::Command;

    let stamp = PathBuf::from("target/audit.timestamp");
    let stamptime = modified(&stamp)?;

    let stampage = stamptime.elapsed().map_err(|e| {
        use std::io::{Error, ErrorKind::Other};
        Error::new(Other, format!("{:?}", e))
    })?;

    // Stamp is expired if we haven't done an audit in too long:
    let expired = stampage >= audit_expiration_duration();

    // Stamp is stale if Cargo.lock has been updated:
    let stale = stamptime < modified("Cargo.lock")?;

    if expired || stale {
        let exe = std::env::current_exe()?;
        let status = Command::new(exe).arg("audit").arg("--force").status()?;

        if status.success() {
            // Touch the timestamp path:
            std::fs::File::create(stamp)?;
        }

        std::process::exit(status.code().unwrap());
    } else {
        // stamp is recent and newer than lockfile:
        println!("Skipped due to recent timestamp: {:?}", stamp);
        Ok(())
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

fn audit_expiration_duration() -> Duration {
    // TODO: Should be const fn.
    Duration::new(AUDIT_EXPIRATION_TIMEOUT_SECS, 0)
}
