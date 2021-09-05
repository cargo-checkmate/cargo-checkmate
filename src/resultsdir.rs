use crate::CMDNAME;
use std::path::PathBuf;

pub fn results_dir() -> PathBuf {
    PathBuf::from(".").join("target").join(CMDNAME)
}
