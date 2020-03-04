#![deny(warnings)]

mod phases;
mod runner;

fn main() -> std::io::Result<()> {
    runner::run(phases::PHASES)
}

#[test]
fn test_fail_injector() {
    use std::env;

    let name = format!("{}-INJECT-TEST-FAILURE", env!("CARGO_PKG_NAME"));
    assert!(std::env::var_os(name).is_none());
}
