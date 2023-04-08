use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};

static BUNDLE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/hook/bundles");

#[derive(Debug)]
pub struct SourceBundle {
    name: &'static str,
    dest: PathBuf,
    executable: bool,
    versions: Vec<&'static [u8]>,
}

/// An internal type for recognition status:
use Recognition::*;

#[derive(PartialEq, Eq)]
enum Recognition {
    Current,
    Old,
    Unrecognized,
}

impl SourceBundle {
    pub fn new(name: &'static str, srcname: &str, dest: PathBuf, executable: bool) -> SourceBundle {
        let versions = load_versions(srcname);

        SourceBundle {
            name,
            dest,
            executable,
            versions,
        }
    }

    pub fn install(&self, force: bool) -> std::io::Result<()> {
        use crate::CMDNAME;
        use FileOrAlreadyExists::*;

        std::fs::create_dir_all(self.dest.parent().unwrap())?;
        match open_file(&self.dest, force)? {
            File(mut f) => {
                use std::io::Write;

                f.write_all(self.latest())?;
                if self.executable {
                    make_executable(f)?;
                }
                println!("{} {} installed: {:?}", CMDNAME, self.name, &self.dest);
                Ok(())
            }
            AlreadyExists => match self.contents_recognized()? {
                Current => {
                    println!(
                        "{} {} already installed: {:?}",
                        CMDNAME, self.name, &self.dest
                    );
                    Ok(())
                }
                Old => {
                    println!("{} {} upgrading from old version.", CMDNAME, self.name,);
                    self.install(true)
                }
                Unrecognized => self.unrecognized_contents(),
            },
        }
    }

    pub fn uninstall(&self, force: bool) -> std::io::Result<()> {
        if force || self.contents_recognized()? != Unrecognized {
            use crate::CMDNAME;
            std::fs::remove_file(&self.dest)?;
            println!("{} {} uninstalled: {:?}", CMDNAME, self.name, &self.dest);
            Ok(())
        } else {
            self.unrecognized_contents()
        }
    }

    fn contents_recognized(&self) -> std::io::Result<Recognition> {
        let found = std::fs::read(&self.dest)?;
        Ok(if found == self.latest() {
            Current
        } else if self.versions.iter().any(|x| **x == found) {
            Old
        } else {
            Unrecognized
        })
    }

    fn unrecognized_contents(&self) -> std::io::Result<()> {
        use crate::{ioerror, CMDNAME};

        println!("{} unrecognized {}: {:?}", CMDNAME, self.name, self.dest);
        Err(ioerror!("Unrecognized {}: {:?}", self.name, self.dest))
    }

    fn latest(&self) -> &'static [u8] {
        self.versions.last().unwrap()
    }
}

#[derive(Debug)]
enum FileOrAlreadyExists {
    File(std::fs::File),
    AlreadyExists,
}

fn open_file(path: &Path, force: bool) -> std::io::Result<FileOrAlreadyExists> {
    use FileOrAlreadyExists::*;

    let mut openopts = std::fs::OpenOptions::new();
    openopts.write(true);

    if !force {
        openopts.create_new(true);
    }

    match openopts.open(path) {
        Ok(f) => Ok(File(f)),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(AlreadyExists),
            _ => Err(e),
        },
    }
}

fn make_executable(f: std::fs::File) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = f.metadata()?.permissions();
    // Set user read/write perms on unix:
    perms.set_mode(perms.mode() | 0o500);
    f.set_permissions(perms)?;
    Ok(())
}

// `load_versions` must not fail, but it may panic if passed incorrect parameters or a malformed
// `hooksrc` directory. A unittest ensures that it does not panic.
fn load_versions(srcname: &str) -> Vec<&'static [u8]> {
    let verdir = BUNDLE_DIR
        .get_dir(srcname)
        .unwrap_or_else(|| panic!("srcname {:?} not found.", srcname));

    let mut vslots = vec![];

    for entry in verdir.entries() {
        let path = entry.path();
        let fname = unwrap_file_name_str(path);

        // Each version must be named `v<N>`:
        let verstr = fname.strip_prefix('v').unwrap_or_else(|| {
            panic!(
                "expected hooksrc version file `v<N>`; found {:?}",
                path.display()
            )
        });

        let version = {
            use std::str::FromStr;

            usize::from_str(verstr).unwrap_or_else(|_err| {
                panic!(
                    "expected hooksrc version file `v<N>`; found {:?}",
                    path.display()
                )
            })
        };

        // Each version must be a file:
        let file = entry
            .as_file()
            .unwrap_or_else(|| panic!("Expected a file: {:?}", path.display()));

        if version >= vslots.len() {
            vslots.resize(version + 1, None);
        }

        vslots[version] = Some(file.contents());
    }

    if vslots.is_empty() {
        panic!("No versions found for {:?}", srcname);
    }
    let vslotslen = vslots.len();

    // Any empty slots are an error:
    let mut versions = vec![];
    for (version, vslot) in vslots.into_iter().enumerate() {
        let contents = vslot.unwrap_or_else(|| {
            panic!(
                "Gap in versions for {:?}; version {} absent.",
                srcname, version
            )
        });
        versions.push(contents);
    }
    assert_eq!(vslotslen, versions.len());

    versions
}

fn unwrap_file_name_str(p: &Path) -> &str {
    p.file_name()
        .unwrap_or_else(|| panic!("Path {:?} has no filename.", p.display()))
        .to_str()
        .unwrap_or_else(|| panic!("Path {:?} filename is not utf.", p.display()))
}

#[test]
fn test_load_versions() {
    for entry in BUNDLE_DIR.entries() {
        let srcname = unwrap_file_name_str(entry.path());
        assert!(
            srcname == "git-hook.pre-commit" || srcname == "github-ci.yaml",
            "Unexpected source bundle: {:?}",
            srcname
        );
        // This should not panic:
        load_versions(srcname);
    }
}
