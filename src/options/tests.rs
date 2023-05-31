use super::{Options, Subcommand::Phase};
use crate::phase::Phase::Clippy;
use test_case::test_case;

#[test_case(
    &["cargo-checkmate"]
    => Ok(Options { cmd: None})
    ; "checkmate-exec-no-args"
)]
#[test_case(
    &["cargo", "checkmate"]
    => Ok(Options { cmd: None})
    ; "cargo-checkmate-no-args"
)]
#[test_case(
    &["cargo-checkmate", "clippy"]
    => Ok(Options { cmd: Some(Phase(Clippy))})
    ; "checkmate-clippy"
)]
#[test_case(
    &["cargo", "checkmate", "clippy"]
    => Ok(Options { cmd: Some(Phase(Clippy))})
    ; "cargo-checkmate-clippy"
)]
#[test_case(
    &["/path/to/cargo", "checkmate", "clippy"]
    => Ok(Options { cmd: Some(Phase(Clippy))})
    ; "cargopath-checkmate-clippy"
)]
// This case should not occur in the wild, but will still parse:
#[test_case(
    &["cargo", "../foo/weird/checkmate", "clippy"]
    => Ok(Options { cmd: Some(Phase(Clippy))})
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
