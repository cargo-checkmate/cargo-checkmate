# cargo-checkmate

Perform a series of useful checks out of the box. `cargo-checkmate` ensures your project builds, tests pass, has good format, doesn't have dependencies with known vulnerabilities, and so on.

The philosophy is that you can just run it without configuration on most crates to catch as many issues as possible (with low very low false-positives). The rationale behind not having configuration is that checkmate failures should be the same for all developers (for a given version of `cargo-checkmate`) regardless of individual developer configurations.

## How to use it

``` bash
$ cargo install cargo-checkmate
...

$ cd /path/to/your/crate

$ cargo checkmate

running 6 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... ok.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate doc... ok.
cargo-checkmate audit... ok.

cargo-checkmate result: ok. 6 passed; 0 failed
```

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

