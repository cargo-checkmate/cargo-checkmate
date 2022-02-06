use crate::IOResult;

pub fn run(args: &[&str]) -> IOResult<String> {
    use crate::ioerror;
    use std::io::Write;
    use std::process::Command;

    let gitout = Command::new("git").args(args).output()?;

    let errbytes = &gitout.stderr[..];

    // Echo any stderr output:
    std::io::stderr().write_all(errbytes)?;

    if gitout.status.success() && errbytes.is_empty() {
        String::from_utf8(gitout.stdout).map_err(|e| ioerror!("{:?} git-dir not utf8", e))
    } else {
        Err(ioerror!(
            "git {} exit {}{}",
            args.join(" "),
            gitout
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "???".to_string()),
            if errbytes.is_empty() {
                ""
            } else {
                " with stderr noise."
            }
        ))
    }
}
