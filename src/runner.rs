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
    cratedir: PathBuf,
    rellogdir: PathBuf,
    passes: Vec<PhaseResult>,
    fails: Vec<PhaseResult>,
}

impl Runner {
    fn new() -> Result<Runner> {
        let cratedir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .ok()
                .unwrap_or(".".to_string()),
        );

        let rellogdir: PathBuf = [".", "target", CMDNAME, "logs"].iter().collect();

        {
            let logdir = &cratedir.join(&rellogdir);
            if logdir.exists() {
                println!("Removing prior log directory: {}", &rellogdir.display());
                std::fs::remove_dir_all(logdir)?;
            }
            std::fs::create_dir_all(logdir)?;
        }

        Ok(Runner {
            cratedir: cratedir,
            rellogdir: rellogdir,
            passes: vec![],
            fails: vec![],
        })
    }

    fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> Result<()> {
        use std::process::Command;

        let (phasename, exec) = if subcommand == "checkmate" {
            assert!(args.len() > 0);
            (args[0], std::env::current_exe()?)
        } else {
            (subcommand, PathBuf::from("cargo"))
        };

        print!("{} {}... ", CMDNAME, phasename);

        {
            use std::io::Write;
            std::io::stdout().flush()?;
        }

        let output = Command::new(exec).arg(subcommand).args(args).output()?;

        let reloutlog = self.rellog_path(phasename, "stdout");
        let relerrlog = self.rellog_path(phasename, "stderr");

        {
            use std::fs::File;
            use std::io::Write;

            File::create(&self.cratedir.join(&relerrlog))?.write_all(&output.stderr)?;
            File::create(&self.cratedir.join(&reloutlog))?.write_all(&output.stdout)?;
        }

        let results = if output.status.success() {
            println!("ok.");
            &mut self.passes
        } else {
            println!("FAILED.");
            &mut self.fails
        };

        results.push(PhaseResult::new(
            phasename,
            self.cratedir.clone(),
            reloutlog,
            relerrlog,
        ));

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

    fn rellog_path(&self, phasename: &str, outkind: &str) -> PathBuf {
        self.rellogdir.join(&format!("{}.{}", phasename, outkind))
    }
}
