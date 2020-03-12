#![deny(warnings)]

mod phases;
mod runner;
mod subcommands;

#[cfg(test)]
mod tfi;

const CMDNAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let argrefs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let subargs = if argrefs.get(1) == Some(&"checkmate") {
        &argrefs[2..]
    } else {
        &argrefs[1..]
    };

    match subargs {
        &[] => runner::run(phases::PHASES),
        &["audit"] => subcommands::audit(),
        _ => {
            use std::io::{Error, ErrorKind};

            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown args: {:?}", subargs),
            ))
        }
    }
}
