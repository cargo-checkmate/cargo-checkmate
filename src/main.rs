#![deny(warnings)]

mod phase;

fn main() -> Result<(), std::io::Error> {
    let mut runner = phase::Runner::new();
    runner.run_phase("build", &[])?;
    runner.run_phase("test", &[])?;
    runner.exit();
    Ok(())
}
