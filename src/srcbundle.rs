use crate::IOResult;
use std::path::Path;

#[derive(Debug)]
pub enum FileOrAlreadyExists {
    File(std::fs::File),
    AlreadyExists,
}

pub fn open_if_non_existent(path: &Path) -> IOResult<FileOrAlreadyExists> {
    use FileOrAlreadyExists::*;

    let openres = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path);

    match openres {
        Ok(f) => Ok(File(f)),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(AlreadyExists),
            _ => Err(e),
        },
    }
}
