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

        self.write_log(subcommand, "stderr", &output.stderr)?;
        self.write_log(subcommand, "stdout", &output.stdout)?;

        if output.status.success() {
            println!("{}", "ok.");
        } else {
            use std::io::Write;

            println!("{}", "FAILED.");
            self.success = false;
            std::io::stdout().write_all(&output.stdout)?;
            std::io::stdout().write_all(&output.stderr)?;
        }

        Ok(())
    }

    pub fn exit(&self) {
        std::process::exit(if self.success { 0 } else { 1 })
    }

    fn write_log(&self, subcommand: &str, outkind: &str, bytes: &[u8]) -> std::io::Result<()> {
        use std::io::Write;

        let outpath = self.logdir.join(&format!("{}.{}", subcommand, outkind));
        let mut f = std::fs::File::create(outpath)?;
        f.write_all(bytes)?;
        Ok(())
    }
}
