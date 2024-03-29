mod phaseresult;

use self::phaseresult::PhaseResult;
use crate::duration::DurationTracker;
use crate::phase::Phase;
use anyhow::Result;
use anyhow_std::PathAnyhow;
use colored::Colorize;
use std::path::PathBuf;

pub struct Runner {
    logdir: PathBuf,
    passes: Vec<PhaseResult>,
    fails: Vec<PhaseResult>,
    dt: DurationTracker,
}

impl Runner {
    pub fn new() -> Result<Runner> {
        let logdir = crate::results_dir().join("logs");

        if logdir.exists() {
            println!("Removing prior log directory: {}", &logdir.display());
            logdir.remove_dir_all_anyhow()?;
        }
        logdir.create_dir_all_anyhow()?;

        Ok(Runner {
            logdir,
            passes: vec![],
            fails: vec![],
            dt: DurationTracker::start(),
        })
    }

    pub fn run_phase(&mut self, phase: &Phase) -> Result<()> {
        use anyhow_std::CommandAnyhow;
        use std::process::Command;

        let exec = std::env::current_exe()?;
        let phasename = &phase.to_string();
        let phasename_width = Phase::max_phase_name_length();
        let pkgname = env!("CARGO_PKG_NAME");

        print!("{pkgname} run {phasename:phasename_width$} ... ");

        {
            use std::io::Write;
            std::io::stdout().flush()?;
        }

        let dt = DurationTracker::start();
        let output = Command::new(exec)
            .arg("run")
            .arg(phasename)
            .output_anyhow()?;

        let duration = dt.finish().format_seconds();

        let relerrlog = self.rellog_path(phasename, "stderr");
        let reloutlog = self.rellog_path(phasename, "stdout");

        relerrlog.write_anyhow(&output.stderr)?;
        reloutlog.write_anyhow(&output.stdout)?;

        let results = if output.status.success() {
            if output.stdout.starts_with(b"skipped:\n") {
                println!("{:7} in {}", "skipped".green(), duration);
            } else {
                println!("{:7} in {}", "ok".green(), duration);
            }
            &mut self.passes
        } else {
            println!("{:7} in {}", "FAILED".red(), duration);
            &mut self.fails
        };

        results.push(PhaseResult::new(phasename, reloutlog, relerrlog));

        Ok(())
    }

    pub fn exit(self) -> Result<()> {
        let duration = self.dt.finish();
        let passcount = self.passes.len();
        let failcount = self.fails.len();

        let (exitstatus, label) = if failcount == 0 {
            (0, "ok".green())
        } else {
            println!("\nfailures:\n");

            for fres in self.fails {
                fres.display()?;
            }

            (1, "FAILED".red())
        };

        println!(
            "\n{} result: {}. {} passed; {} failed; total duration {}",
            env!("CARGO_PKG_NAME"),
            label,
            passcount,
            failcount,
            duration.format_human(),
        );

        std::process::exit(exitstatus);
    }

    fn rellog_path(&self, phasename: &str, outkind: &str) -> PathBuf {
        self.logdir.join(format!("{}.{}", phasename, outkind))
    }
}
