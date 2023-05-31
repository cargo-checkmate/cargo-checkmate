use super::{Options, Subcommand::Run};
use crate::phase::Phase::Clippy;
use test_case::test_case;

const RUN_CLIPPY: Options = Options {
    cmd: Some(Run {
        phase: Some(Clippy),
    }),
};

#[test_case(
    &["cargo-checkmate"]
    => Ok(Options { cmd: None })
    ; "checkmate-exec-no-args"
)]
#[test_case(
    &["cargo", "checkmate"]
    => Ok(Options { cmd: None })
    ; "cargo-checkmate-no-args"
)]
#[test_case(
    &["cargo", "checkmate", "run"]
    => Ok(Options { cmd: Some(Run { phase: None })})
    ; "cargo-checkmate-run"
)]
#[test_case(
    &["cargo-checkmate", "run", "clippy"]
    => Ok(RUN_CLIPPY)
    ; "checkmate-clippy"
)]
#[test_case(
    &["cargo", "checkmate", "run", "clippy"]
    => Ok(RUN_CLIPPY)
    ; "cargo-checkmate-clippy"
)]
#[test_case(
    &["/path/to/cargo", "checkmate", "run", "clippy"]
    => Ok(RUN_CLIPPY)
    ; "cargopath-checkmate-clippy"
)]
// This case should not occur in the wild, but will still parse:
#[test_case(
    &["cargo", "../foo/weird/checkmate", "run", "clippy"]
    => Ok(RUN_CLIPPY)
    ; "cargo-checkmateweirdpath-clippy"
)]
#[test_case(
    &["cargo", "checkmate", "--help"] 
    => Err(env!("CARGO_PKG_DESCRIPTION").trim().to_string())
    ; "cargo-checkmate-help"
)]
#[test_case(
    &[]
    => Err(r#"error: expecting one of ["cargo", "cargo-checkmate"]; found nothing"#.to_string())
    ; "empty-args"
)]
#[test_case(
    &["foob"]
    => Err(r#"error: expecting one of ["cargo", "cargo-checkmate"]; found "foob""#.to_string())
    ; "foob-bin"
)]
#[test_case(
    &["cargo", "bork"]
    => Err(r#"error: expecting one of ["checkmate"]; found "bork""#.to_string())
    ; "cargo-bork-bin"
)]
fn parse(args: &[&str]) -> Result<Options, String> {
    Options::try_parse_args(args).map_err(|e| {
        let fullerr = e.to_string();

        fullerr
            .as_str()
            .split_once("\n\n")
            .map(|(prefix, _)| prefix.to_string())
            .unwrap_or(fullerr)
    })
}
