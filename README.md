# cargo-checkmate

Perform a series of useful checks out of the box. `cargo-checkmate` ensures your project builds, tests pass, has good format, doesn't have dependencies with known vulnerabilities, and so on.

The philosophy is that you can just run it without configuration on most crates to catch as many issues as possible (with low very low false-positives). The rationale behind not having configuration is to know if two different crates pass the same version of `cargo-checkmate` they pass the same checks. Also, this helps ensure bugs are likely shared by most users rather than only present for unusual configurations.

## How to use it:

``` bash
$ cargo install cargo-checkmate
...

$ cd /path/to/your/crate

$ cargo-checkmate

running 5 cargo-checkmate phases
cargo-checkmate check... ok.
cargo-checkmate format... ok.
cargo-checkmate build... ok.
cargo-checkmate test... ok.
cargo-checkmate audit... ok.

cargo-checkmate result: ok. 5 passed; 0 failed
```
