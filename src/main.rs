#![deny(warnings)]

mod phases;
mod runner;

fn main() -> std::io::Result<()> {
    runner::run(phases::PHASES)
}
