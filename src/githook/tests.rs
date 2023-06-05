use indoc::indoc;
use test_case::test_case;

#[test_case(
    indoc! { r#"
        ?? src/githook/tests.rs
         M src/githook.rs
    "# }
    => Ok(vec![
        "src/githook/tests.rs".to_string(),
        "src/githook.rs".to_string(),
    ])
)]
#[test_case(
    indoc! { r#"
         M src/githook.rs
        A  src/githook/tests.rs
    "# }
    => Ok(vec![
        "src/githook.rs".to_string(),
    ])
)]
fn parse_git_status_porcelain(input: &str) -> Result<Vec<String>, String> {
    super::parse_git_status_porcelain(input).map_err(|e| format!("{e:#}"))
}

#[test_case(
    []
    => Ok(())
)]
#[test_case(
    ["foo", "bar", "blah"]
    => Err(indoc! { r#"
        The checkmate tool validates the current filesystem, but something different
        is staged for git commit. Since checkmate cannot validate the commit contents,
        this git hook is aborting.

        To save unstaged changes for after a commit, see `git stash`.


        Caused by:
            These files have changes that are not staged in git:
              foo
              bar
              blah
    "# }.trim_end().to_string())
    ; "foo-bar-blah-error"
)]
fn git_unstaged_to_error_message<const K: usize>(dirty: [&str; K]) -> Result<(), String> {
    super::git_unstaged_to_error_message(dirty.into_iter().map(|s| s.to_string()).collect())
        .map_err(|e| format!("{e:?}"))
}
