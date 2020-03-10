#![deny(warnings)]

mod phases;
mod runner;

#[cfg(test)]
mod tfi;

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    runner::run(phases::PHASES)
}
