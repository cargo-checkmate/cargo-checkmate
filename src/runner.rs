mod phaseresult;

use self::phaseresult::PhaseResult;
use crate::phase::Phase;
use anyhow::Result;
use anyhow_std::PathAnyhow;
use colored::Colorize;
use std::path::PathBuf;

pub struct Runner {
    logdir: PathBuf,
    passes: Vec<PhaseResult>,
    fails: Vec<PhaseResult>,
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
        })
    }

    pub fn run_phase(&mut self, phase: &Phase) -> Result<()> {
        use anyhow_std::CommandAnyhow;
        use std::process::Command;

        let exec = std::env::current_exe()?;
        let phasename = &phase.to_string();
        let mut padded = phasename.to_string();

        // Pad the phase name for column alignment:
        for _ in phasename.chars().count()..Phase::max_phase_name_length() {
            padded.push(' ');
        }

        print!("{} run {} ... ", env!("CARGO_PKG_NAME"), padded);

        {
            use std::io::Write;
            std::io::stdout().flush()?;
        }

        let output = Command::new(exec)
            .arg("run")
            .arg(phasename)
            .output_anyhow()?;

        let reloutlog = self.rellog_path(phasename, "stdout");
        let relerrlog = self.rellog_path(phasename, "stderr");

        {
            use std::fs::File;
            use std::io::Write;

            File::create(&relerrlog)?.write_all(&output.stderr)?;
            File::create(&reloutlog)?.write_all(&output.stdout)?;
        }

        let results = if output.status.success() {
            if output.stdout.starts_with(b"skipped:\n") {
                println!("{}.", "skipped".green());
            } else {
                println!("{}.", "ok".green());
            }
            &mut self.passes
        } else {
            println!("{}.", "FAILED".red());
            &mut self.fails
        };

        results.push(PhaseResult::new(phasename, reloutlog, relerrlog));

        Ok(())
    }

    pub fn exit(self) -> Result<()> {
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
            "\n{} result: {}. {} passed; {} failed",
            env!("CARGO_PKG_NAME"),
            label,
            passcount,
            failcount,
        );

        std::process::exit(exitstatus);
    }

    fn rellog_path(&self, phasename: &str, outkind: &str) -> PathBuf {
        self.logdir.join(format!("{}.{}", phasename, outkind))
    }
}
