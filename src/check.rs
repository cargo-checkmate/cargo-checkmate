use crate::{invalid_input, IOResult};
use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, IntoEnumIterator)]
pub enum Check {
    Audit,
    Build,
    Check,
    Everything,
    Format,
    Test,
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
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
