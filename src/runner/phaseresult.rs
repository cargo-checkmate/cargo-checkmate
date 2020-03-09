use std::io::Result;
use std::path::PathBuf;

mod indenter;

pub struct PhaseResult {
    name: String,
    cratedir: PathBuf,
    outlog: PathBuf,
    errlog: PathBuf,
}

impl PhaseResult {
    pub fn new(name: &str, cratedir: PathBuf, outlog: PathBuf, errlog: PathBuf) -> PhaseResult {
        let name = String::from(name);
        PhaseResult {
            name,
            cratedir,
            outlog,
            errlog,
        }
    }

    pub fn display(self) -> Result<()> {
        use self::indenter::Indenter;
        use std::fs::File;
        use std::io::{copy, stdout};

        let mut empty = true;

        println!("---- {} {} ----", crate::CMDNAME, self.name);
        for rellogpath in &[self.outlog, self.errlog] {
            let logpath = self.cratedir.join(rellogpath);
            if logpath.metadata()?.len() > 0 {
                empty = false;
                println!("+ {}:", rellogpath.display());
                copy(&mut File::open(logpath)?, &mut Indenter::from(stdout()))?;
            }
        }

        if empty {
            println!("+ No output.");
        }

        Ok(())
    }
}
