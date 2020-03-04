use std::io::Result;
use std::path::PathBuf;

mod indenter;

pub fn run_phases(phases: &[(&str, &[&str])]) -> Result<()> {
    let mut runner = Runner::new()?;

    println!(
        "\nrunning {} {} phases",
        phases.len(),
        env!("CARGO_PKG_NAME")
    );

    for (name, args) in phases {
        runner.run_phase(name, args)?;
    }

    runner.exit()
}

struct Runner {
    logdir: PathBuf,
    passcount: usize,
    failcount: usize,
}

impl Runner {
    fn new() -> Result<Runner> {
        let logdir = [
            &std::env::var("CARGO_MANIFEST_DIR")
                .ok()
                .unwrap_or(".".to_string()),
            "target",
            env!("CARGO_PKG_NAME"),
            "logs",
        ]
        .iter()
        .collect();

        std::fs::create_dir_all(&logdir)?;

        Ok(Runner {
            logdir: logdir,
            passcount: 0,
            failcount: 0,
        })
    }

    fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> Result<()> {
        use std::process::Command;

        print!("{} {}... ", env!("CARGO_PKG_NAME"), subcommand);
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
            self.passcount += 1;
            println!("ok.");
        } else {
            use self::indenter::Indenter;
            use std::io::Write;

            self.failcount += 1;
            println!("FAILED:");

            let mut f = Indenter::from(std::io::stdout());

            println!("+ {}:", outlog.display());
            f.write_all(&output.stdout)?;

            println!("+ {}:", errlog.display());
            f.write_all(&output.stderr)?;
        }

        Ok(())
    }

    fn exit(&self) -> Result<()> {
        println!(
            "\n{} result: {}. {} passed; {} failed",
            env!("CARGO_PKG_NAME"),
            if self.success() { "ok" } else { "FAILED" },
            self.passcount,
            self.failcount
        );
        std::process::exit(if self.success() { 0 } else { 1 });
    }

    fn success(&self) -> bool {
        self.failcount == 0
    }

    fn log_path(&self, subcommand: &str, outkind: &str) -> PathBuf {
        self.logdir.join(&format!("{}.{}", subcommand, outkind))
    }
}
