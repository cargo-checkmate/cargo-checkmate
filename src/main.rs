#![deny(warnings)]

mod phase;

const PHASES: &[(&str, &[&str])] = &[("build", &[]), ("test", &[])];

fn main() -> std::io::Result<()> {
    phase::run_phases(PHASES)
}
