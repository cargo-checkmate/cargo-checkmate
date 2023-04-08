# cargo-checkmate
[![CICD](https://github.com/cargo-checkmate/cargo-checkmate/actions/workflows/cargo-checkmate.yaml/badge.svg)](https://github.com/cargo-checkmate/cargo-checkmate/actions/workflows/cargo-checkmate.yaml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/cargo-checkmate)
![Stars](https://img.shields.io/github/stars/cargo-checkmate/cargo-checkmate)

Perform a series of useful checks out of the box. `cargo-checkmate` ensures your project builds, tests pass, has good format, doesn't have dependencies with known vulnerabilities, and so on.

The philosophy is that you can just run it without configuration on most crates to catch as many issues as possible (with low false-positives). The rationale behind not having configuration is that checkmate failures should be the same for all developers (for a given version of `cargo-checkmate`) regardless of individual developer configurations.

## How to use it

``` bash
$ cargo install cargo-checkmate
...

$ cd /path/to/your/crate

$ cargo checkmate

running 7 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... ok.
cargo-checkmate clippy... ok.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate doc... ok.
cargo-checkmate audit... ok.

cargo-checkmate result: ok. 7 passed; 0 failed
```

### git hook

If your crate is in a git repo, you can verify each commit follows `cargo checkmate` checks by running it in a `pre-commit` git hook. You can install a pre-bundled git hook that does precisely that:

```
$ cargo checkmate git-hook install
cargo-checkmate git-hook installed: ".git/hooks/pre-commit"
```

Now commits are checked:
```
$ touch foo
$ git add foo
$ git commit -m 'Demo cargo checkmate git-hook usage.'

cargo checkmate git-hook:
Removing prior log directory: ./target/cargo-checkmate/logs

running 6 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... ok.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate doc... ok.
cargo-checkmate audit... ok.

cargo-checkmate result: ok. 6 passed; 0 failed
[master 6e3230a] Demo cargo checkmate git-hook usage.
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 src/foo
```

If you want to intentionally skip the checks for a commit, git provides the `git commit --no-verify` flag. If you change your mind, you can likewise uninstall:

```
$ cargo checkmate git-hook uninstall
cargo-checkmate git-hook uninstalled: ".git/hooks/pre-commit"
```

Both install & uninstall try to be very careful about not clobbering any unrecognized `pre-commit` hook in case you already have a custom one:

```
$ cargo checkmate git-hook install
cargo-checkmate git-hook installed: ".git/hooks/pre-commit"

$ cargo checkmate git-hook install
cargo-checkmate git-hook already installed: ".git/hooks/pre-commit"

$ echo 'blah' > .git/hooks/pre-commit

$ cargo checkmate git-hook uninstall
cargo-checkmate unrecognized git-hook: ".git/hooks/pre-commit"
Error: Custom { kind: Other, error: "Unrecongized git-hook: \".git/hooks/pre-commit\"" }

$ cargo checkmate git-hook install
cargo-checkmate unrecognized git-hook: ".git/hooks/pre-commit"
Error: Custom { kind: Other, error: "Unrecongized git-hook: \".git/hooks/pre-commit\"" }
```

### GitHub CI

If you use GitHub, you can install a GitHub Action which runs `cargo-checkmate` on each `push` and `pull_request` event:

```
$ cargo checkmate github-ci install
cargo-checkmate GitHub CI installed: "/path/to/your/crate/.github/workflows/cargo-checkmate.yaml"
```

You must commit this to your repository separately from this command.

The install/uninstall behavior is the same logic as for the `git-hook` subcommand, with care not to delete or overwrite unexpected file contents.

### Logs

Each check phase logs both stdout and stderr into `./target/cargo-checkmate/logs`:

```
$ cat ./target/cargo-checkmate/logs/doc.stderr
 Documenting cargo-checkmate v0.1.2 (/home/user/hack/cargo-checkmate)
    Finished dev [unoptimized + debuginfo] target(s) in 1.67s
```

On each run, any pre-existing logs are first removed, and this fact is reported:

```
$ cargo checkmate
Removing prior log directory: ./target/cargo-checkmate/logs

running 6 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... ok.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate doc... ok.
cargo-checkmate audit... ok.

cargo-checkmate result: ok. 6 passed; 0 failed

```

### Failures

If any phase fails, the stdout/stderr logs are displayed automatically:

```
$ # Introduce a poorly formatted unit test:
$ echo '#[test] fn bad_format() {}' >> src/main.rs
$ cargo checkmate
Removing prior log directory: ./target/cargo-checkmate/logs

running 6 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... FAILED.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate doc... ok.
cargo-checkmate audit... ok.

failures:

---- cargo-checkmate format ----
+ ./target/cargo-checkmate/logs/format.stdout:
| Diff in /home/user/hack/cargo-checkmate/src/main.rs at line 17:
|      let check = Check::parse_args(std::env::args())?;
|      check.execute()
|  }
| -#[test] fn bad_format() {}
| +#[test]
| +fn bad_format() {}
|

cargo-checkmate result: FAILED. 5 passed; 1 failed
```

### Audits

#### Audit Freshness

The `cargo audit` command always fetches an advisory db which requires network access and latency. As an optimization, `cargo-checkmate` skips `cargo audit` if the following conditions are true:

- `Cargo.lock` has not been modified, and
- The timestamp of the last successful run of `cargo audit` is less than three hours old.

This optimizes the use of `cargo-checkmate` especially for the git hook assuming a developer is committing many revisions over a couple of hours.

#### Ignoring vulnerability disclosures

Sometimes your crate's transitive dependencies have vulnerability disclosures that you are aware of, which cannot be resolved with a simple `cargo update`, and which have a minimal impact on your crate's users. In that case, you can instruct `cargo audit` to ignore them, so that they won't block successful completion of `cargo checkmate` in your continuous integration flow.

To do so, configure `cargo audit` for your project to ignore the specific issues. It is good practice to explain in a comment why they are being ignored.

The configuration file is in `.cargo/audit.toml` in your crate's project directory. Currently `cargo checkmate` itself ignores two errors:

```toml
[advisories]
ignore = [
    # `time` localtime_r segfault
    # This is ignored without a clear understanding of the impact on `cargo-checkmate`!
    "RUSTSEC-2020-0071",

    # `chrono` localtime_r segfault
    # This is ignored without a clear understanding of the impact on `cargo-checkmate`!
    "RUSTSEC-2020-0159",
]
```
