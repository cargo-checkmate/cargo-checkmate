use crate::IOResult;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SourceBundle {
    pub name: &'static str,
    pub dest: PathBuf,
    pub contents: &'static [u8],
    pub executable: bool,
}

impl SourceBundle {
    pub fn install(&self) -> IOResult<()> {
        use crate::CMDNAME;
        use FileOrAlreadyExists::*;

        match open_if_non_existent(&self.dest)? {
            File(mut f) => {
                use std::io::Write;

                f.write_all(self.contents)?;
                if self.executable {
                    make_executable(f)?;
                }
                println!("{} {} installed: {:?}", CMDNAME, self.name, &self.dest);
                Ok(())
            }
            AlreadyExists => {
                if contents_recognized(&self.dest, self.contents)? {
                    println!(
                        "{} {} already installed: {:?}",
                        CMDNAME, self.name, &self.dest
                    );
                    Ok(())
                } else {
                    unrecognized_contents(self.name, &self.dest)
                }
            }
        }
    }

    pub fn uninstall(&self) -> IOResult<()> {
        if contents_recognized(&self.dest, self.contents)? {
            use crate::CMDNAME;
            std::fs::remove_file(&self.dest)?;
            println!("{} {} uninstalled: {:?}", CMDNAME, self.name, &self.dest);
            Ok(())
        } else {
            unrecognized_contents(self.name, &self.dest)
        }
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
