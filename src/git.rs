pub fn run(args: &[&str]) -> anyhow::Result<String> {
    use anyhow::Context;
    use anyhow_std::CommandAnyhow;
    use std::io::Write;
    use std::process::Command;

    let gitout = Command::new("git").args(args).output_anyhow()?;

    let errbytes = &gitout.stderr[..];

    // Echo any stderr output:
    std::io::stderr().write_all(errbytes)?;

    if gitout.status.success() && errbytes.is_empty() {
        String::from_utf8(gitout.stdout).with_context(|| "git-dir not utf8".to_string())
    } else {
        Err(anyhow::anyhow!(
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
