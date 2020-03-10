#[test]
fn test_fail_injector() {
    let name = format!("{}-INJECT-TEST-FAILURE", crate::CMDNAME);
    assert!(std::env::var_os(name).is_none());
}
