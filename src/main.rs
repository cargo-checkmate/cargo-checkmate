#![deny(warnings)]

mod phase;

fn main() -> std::io::Result<()> {
    let mut runner = phase::Runner::new()?;
    runner.run_phase("build", &[])?;
    runner.run_phase("test", &[])?;
    runner.exit();
    Ok(())
}
