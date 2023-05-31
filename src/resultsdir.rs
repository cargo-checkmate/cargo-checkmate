use std::path::PathBuf;

pub fn results_dir() -> PathBuf {
    PathBuf::from(".")
        .join("target")
        .join(env!("CARGO_PKG_NAME"))
}
