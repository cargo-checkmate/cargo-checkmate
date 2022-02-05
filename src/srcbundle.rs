use crate::IOResult;
use std::path::Path;

pub fn install(name: &str, dest: &Path, contents: &[u8], executable: bool) -> IOResult<()> {
    use crate::CMDNAME;
    use FileOrAlreadyExists::*;

    match open_if_non_existent(dest)? {
        File(mut f) => {
            use std::io::Write;

            f.write_all(contents)?;
            if executable {
                make_executable(f)?;
            }
            println!("{} {} installed: {:?}", CMDNAME, name, dest);
            Ok(())
        }
        AlreadyExists => {
            if contents_recognized(dest, contents)? {
                println!("{} {} already installed: {:?}", CMDNAME, name, dest);
                Ok(())
            } else {
                unrecognized_contents(name, dest)
            }
        }
    }
}

pub fn uninstall(name: &str, dest: &Path, contents: &[u8]) -> IOResult<()> {
    if contents_recognized(dest, contents)? {
        use crate::CMDNAME;
        std::fs::remove_file(dest)?;
        println!("{} {} uninstalled: {:?}", CMDNAME, name, dest);
        Ok(())
    } else {
        unrecognized_contents(name, dest)
    }
}

#[derive(Debug)]
enum FileOrAlreadyExists {
    File(std::fs::File),
    AlreadyExists,
}

fn open_if_non_existent(path: &Path) -> IOResult<FileOrAlreadyExists> {
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

fn make_executable(f: std::fs::File) -> IOResult<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = f.metadata()?.permissions();
    // Set user read/write perms on unix:
    perms.set_mode(perms.mode() | 0o500);
    f.set_permissions(perms)?;
    Ok(())
}

fn contents_recognized(dest: &Path, contents: &[u8]) -> IOResult<bool> {
    let found = std::fs::read(dest)?;
    Ok(found == contents)
}

fn unrecognized_contents(name: &str, dest: &Path) -> IOResult<()> {
    use crate::{ioerror, CMDNAME};

    println!("{} unrecognized {}: {:?}", CMDNAME, name, dest);
    Err(ioerror!("Unrecongized {}: {:?}", name, dest))
}
