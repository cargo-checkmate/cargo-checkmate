use super::{Options, Subcommand::Phase};
use crate::phase::Phase::Clippy;
use test_case::test_case;

#[test_case(
    &[]
    => Ok(Options { cmd: None})
    ; "empty-args"
)]
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
    &["cargo", "checkmate", "--help"] 
    => Err(format!(
        indoc::indoc! { r#"
            {} {}
            checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks.
        "# },
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ).trim().to_string())
    ; "cargo-checkmate-help"
)]
fn parse(args: &[&str]) -> Result<Options, String> {
    Options::try_parse_args(args)
        .map_err(|e| e.to_string().split_once("\n\n").unwrap().0.to_string())
}
