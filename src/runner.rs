use crate::CMDNAME;
use std::io::Result;
use std::path::PathBuf;

mod phaseresult;
use phaseresult::PhaseResult;

pub fn run(phases: &[(&str, &[&str])]) -> Result<()> {
    let mut runner = Runner::new()?;

    println!("\nrunning {} {} phases", phases.len(), CMDNAME,);

    for (name, args) in phases {
        runner.run_phase(name, args)?;
    }

    runner.exit()
}

struct Runner {
    repodir: PathBuf,
    rellogdir: PathBuf,
    passes: Vec<PhaseResult>,
    fails: Vec<PhaseResult>,
}

impl Runner {
    fn new() -> Result<Runner> {
        let repodir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .ok()
                .unwrap_or(".".to_string()),
        );

        let rellogdir = [".", "target", CMDNAME, "logs"].iter().collect();

        let myself = Runner {
            repodir: repodir,
            rellogdir: rellogdir,
            passes: vec![],
            fails: vec![],
        };

        std::fs::create_dir_all(&myself.logdir())?;

        Ok(myself)
    }

    fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> Result<()> {
        use std::process::Command;

        print!("{} {}... ", CMDNAME, subcommand);
        let output = Command::new("cargo").arg(subcommand).args(args).output()?;

        let outlog = self.log_path(subcommand, "stdout");
        let errlog = self.log_path(subcommand, "stderr");

        {
            use std::fs::File;
            use std::io::Write;

            File::create(&errlog)?.write_all(&output.stderr)?;
            File::create(&outlog)?.write_all(&output.stdout)?;
        }

        let results = if output.status.success() {
            println!("ok.");
            &mut self.passes
        } else {
            println!("FAILED.");
            &mut self.fails
        };

        results.push(PhaseResult::new(subcommand, outlog, errlog));

        Ok(())
    }

    fn exit(self) -> Result<()> {
        let passcount = self.passes.len();
        let failcount = self.fails.len();

        let (exitstatus, label) = if failcount == 0 {
            (0, "ok")
        } else {
            println!("\nfailures:\n");

            for fres in self.fails {
                fres.display()?;
            }

            (1, "FAILED")
        };

        println!(
            "\n{} result: {}. {} passed; {} failed",
            CMDNAME, label, passcount, failcount,
        );

        std::process::exit(exitstatus);
    }

    fn log_path(&self, subcommand: &str, outkind: &str) -> PathBuf {
        self.logdir().join(&format!("{}.{}", subcommand, outkind))
    }

    fn logdir(&self) -> PathBuf {
        self.repodir.join(&self.rellogdir)
    }
}
