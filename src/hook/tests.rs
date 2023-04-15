use crate::hook::HookType;

use test_case::test_case;

#[test_case(HookType::All)]
#[test_case(HookType::Git)]
#[test_case(HookType::GithubCI)]
fn source_bundles(ht: HookType) -> anyhow::Result<()> {
    ht.source_bundles()?;
    Ok(())
}
