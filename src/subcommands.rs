use anyhow_std::{CommandAnyhow, PathAnyhow};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime};

const AUDIT_EXPIRATION_TIMEOUT_SECS: u64 = 60 * 60 * 3;

pub fn cargo_builtin(args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new("cargo").args(args).status_anyhow()?;
    status.exit();
}

pub fn audit(force: bool) -> anyhow::Result<()> {
    if force {
        force_audit()
    } else {
        audit_if_necessary()
    }
}

fn force_audit() -> anyhow::Result<()> {
    use abscissa_core::application::Application;
    use cargo_audit::application::APP as AUDIT_APP;

    Application::run(
        &AUDIT_APP,
        vec![String::from("cargo"), String::from("audit")],
    );

    Ok(())
}

fn audit_if_necessary() -> anyhow::Result<()> {
    let stamp = crate::results_dir().join("audit.timestamp");
    {
        let stampdir = stamp.parent_anyhow()?;
        if !stampdir.is_dir() {
            stampdir.create_dir_all_anyhow()?;
        }
    }

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
        let status = Command::new(exe)
            .arg("run")
            .arg("audit")
            .arg("--force")
            .status_anyhow()?;

        if status.success() {
            // Touch the timestamp path:
            std::fs::File::create(stamp)?;
        }

        status.exit();
    } else {
        // stamp is recent and newer than lockfile:
        println!("skipped:\nFound recent timestamp: {:?}", stamp);
        Ok(())
    }
}

fn modified<P: AsRef<Path>>(p: P) -> anyhow::Result<SystemTime> {
    let pref = p.as_ref();
    if pref.exists() {
        pref.metadata_anyhow()?.modified()
    } else {
        Ok(SystemTime::UNIX_EPOCH)
    }
}

fn audit_expiration_duration() -> Duration {
    // TODO: Should be const fn.
    Duration::new(AUDIT_EXPIRATION_TIMEOUT_SECS, 0)
}
