#![deny(warnings)]

mod phases;
mod runner;

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    runner::run(phases::PHASES)
}

#[test]
fn test_fail_injector() {
    let name = format!("{}-INJECT-TEST-FAILURE", CMDNAME);
    assert!(std::env::var_os(name).is_none());
}
