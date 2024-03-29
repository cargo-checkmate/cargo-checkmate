use crate::executable::Executable;

const README: &str = include_str!("../README.md");

#[derive(Debug, PartialEq, Eq, clap::Parser)]
/// Display the project README.md
pub struct Readme {}

impl Executable for Readme {
    fn execute(&self) -> anyhow::Result<()> {
        use std::io::Write;

        std::io::stdout().write_all(README.as_bytes())?;
        Ok(())
    }
}
