pub struct Runner(bool);

impl Runner {
    pub fn new() -> Runner {
        Runner(true)
    }

    pub fn run_phase(&mut self, subcommand: &str, args: &[&str]) -> std::io::Result<()> {
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

    pub fn exit(&self) {
        std::process::exit(if self.0 { 0 } else { 1 })
    }
}
