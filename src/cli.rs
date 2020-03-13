use std::io::Result as IOResult;

pub enum Command {
    Everything,
    Audit,
}

pub fn parse_args<I>(args: I) -> IOResult<Command>
where
    I: IntoIterator<Item = String>,
{
    let mut it = args.into_iter();

    // The first arg is executable name which we ignore.
    it.next();

    let mut optcmd = it.next();
    if optstr(&optcmd) == Some("checkmate") {
        // If executed by cargo rather than directly, the second argument is
        // "checkmate", which we ignore:
        optcmd = it.next();
    }

    let cmd = match optstr(&optcmd) {
        None | Some("everything") => Ok(Command::Everything),
        Some("audit") => Ok(Command::Audit),
        Some(other) => invalid_input("Unknown command", other),
    }?;

    if let Some(junk) = optstr(&it.next()) {
        invalid_input("Unexpected arg", junk)
    } else {
        Ok(cmd)
    }
}

fn optstr(x: &Option<String>) -> Option<&str> {
    x.as_ref().map(String::as_str)
}

fn invalid_input<T>(reason: &str, input: &str) -> IOResult<T> {
    use std::io::{Error, ErrorKind};

    Err(Error::new(
        ErrorKind::InvalidInput,
        format!("{}: {:?}", reason, input),
    ))
}
