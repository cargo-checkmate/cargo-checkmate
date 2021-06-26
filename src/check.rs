use crate::{invalid_input, IOResult};
use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, IntoEnumIterator)]
pub enum Check {
    Everything,
    Check,
    Format,
    Build,
    Test,
    Doc,
    Audit,
}

impl Check {
    pub fn parse_args<I>(args: I) -> IOResult<Check>
    where
        I: IntoIterator<Item = String>,
    {
        fn optstr(x: &Option<String>) -> Option<&str> {
            x.as_ref().map(String::as_str)
        }

        let mut it = args.into_iter();

        // The first arg is executable name which we ignore.
        it.next();

        let mut optcmd = it.next();
        if optstr(&optcmd) == Some("checkmate") {
            // If executed by cargo rather than directly, the second argument is
            // "checkmate", which we ignore:
            optcmd = it.next();
        }

        let check = match optstr(&optcmd) {
            None => Ok(Check::Everything),
            Some(checkname) => Check::parse(checkname),
        }?;

        if let Some(junk) = optstr(&it.next()) {
            invalid_input("Unexpected arg", junk)
        } else {
            Ok(check)
        }
    }

    fn parse(checkname: &str) -> IOResult<Check> {
        for check in Check::into_enum_iter() {
            if format!("{}", check) == checkname {
                return Ok(check);
            }
        }

        invalid_input("Unknown check", checkname)
    }

    pub fn execute(&self) -> IOResult<()> {
        use crate::subcommands::{audit, cargo_builtin};

        match self {
            Check::Everything => self.execute_everything(),
            Check::Audit => audit(),
            Check::Build => cargo_builtin(&["build"]),
            Check::Check => cargo_builtin(&["check"]),
            Check::Doc => cargo_builtin(&["doc"]),
            Check::Format => cargo_builtin(&["fmt", "--", "--check"]),
            Check::Test => cargo_builtin(&["test"]),
        }
    }

    fn execute_everything(&self) -> IOResult<()> {
        use crate::runner::Runner;

        let mut runner = Runner::new()?;

        println!(
            "\nrunning {} {} phases",
            Check::VARIANT_COUNT - 1,
            crate::CMDNAME
        );

        for check in Check::into_enum_iter() {
            match check {
                Check::Everything => {}
                _ => runner.run_check(&format!("{}", check))?,
            }
        }

        runner.exit()
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
