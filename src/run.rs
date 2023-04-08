/// Run the cli main process
pub fn run() -> crate::IOResult<()> {
    use crate::executable::Executable;
    use crate::options::Options;

    crate::cdcrate::change_directory_to_crate_root()?;
    let opts = Options::parse_args();
    opts.execute()
}
