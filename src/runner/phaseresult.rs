use anyhow::Result;
use std::path::PathBuf;

mod indenter;

pub struct PhaseResult {
    name: String,
    outlog: PathBuf,
    errlog: PathBuf,
}

impl PhaseResult {
    pub fn new(name: &str, outlog: PathBuf, errlog: PathBuf) -> PhaseResult {
        let name = String::from(name);
        PhaseResult {
            name,
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
            if rellogpath.metadata()?.len() > 0 {
                empty = false;
                println!("+ {}:", rellogpath.display());
                copy(&mut File::open(rellogpath)?, &mut Indenter::from(stdout()))?;
            }
        }

        if empty {
            println!("+ No output.");
        }

        Ok(())
    }
}
