#![deny(warnings)]

fn main() -> Result<(), std::io::Error> {
    let mut runner = PhaseRunner::new();
    runner.run_phase("test", &[])?;
    runner.exit();
    Ok(())
}

struct PhaseRunner(bool);

impl PhaseRunner {
    fn new() -> PhaseRunner {
        PhaseRunner(true)
    }

    fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> std::io::Result<()> {
        use std::process::Command;

        print!("Phase {}... ", subcommand);
        let output = Command::new("cargo").arg(subcommand).args(args).output()?;

        if output.status.success() {
            println!("{}", "ok.");
        } else {
            println!("{}", "FAILED.");
            self.0 = false;
        }

        Ok(())
    }

    fn exit(&self) {
        std::process::exit(if self.0 { 0 } else { 1 })
    }
}
