use crate::check::Check;
use crate::IOResult;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    setting = AppSettings::NoBinaryName,
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Options {
    #[structopt(subcommand)]
    cmd: Option<Check>,
}

impl Options {
    pub fn parse_args() -> Options {
        let mut it = std::env::args().peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().map(|s| s.as_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        Options::from_iter(it)
    }

    pub fn execute(&self) -> IOResult<()> {
        self.cmd.as_ref().unwrap_or(&Check::Everything).execute()
    }
}
