use std::path::PathBuf;

pub struct Runner {
    success: bool,
    logdir: PathBuf,
}

impl Runner {
    pub fn new() -> std::io::Result<Runner> {
        let logdir = [
            &std::env::var("CARGO_MANIFEST_DIR")
                .ok()
                .unwrap_or(".".to_string()),
            "target",
            "cargo-catt",
            "logs",
        ]
        .iter()
        .collect();

        std::fs::create_dir_all(&logdir)?;

        Ok(Runner {
            success: true,
            logdir: logdir,
        })
    }

    pub fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> std::io::Result<()> {
        use std::process::Command;

        print!("Phase {}... ", subcommand);
        let output = Command::new("cargo").arg(subcommand).args(args).output()?;

        let outlog = self.log_path(subcommand, "stdout");
        let errlog = self.log_path(subcommand, "stderr");

        {
            use std::fs::File;
            use std::io::Write;

            File::create(&errlog)?.write_all(&output.stderr)?;
            File::create(&outlog)?.write_all(&output.stdout)?;
        }

        if output.status.success() {
            println!("ok.");
        } else {
            use crate::indenter::Indenter;
            use std::io::Write;

            self.success = false;
            println!("FAILED:");

            let mut f = Indenter::from(std::io::stdout());

            println!("{}:", outlog.display());
            f.write_all(&output.stdout)?;

            println!("{:?}:", errlog.display());
            f.write_all(&output.stderr)?;
        }

        Ok(())
    }

    pub fn exit(&self) {
        std::process::exit(if self.success { 0 } else { 1 })
    }

    fn log_path(&self, subcommand: &str, outkind: &str) -> PathBuf {
        self.logdir.join(&format!("{}.{}", subcommand, outkind))
    }
}
