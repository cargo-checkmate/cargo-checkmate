use super::Indenter;
use std::io::Write;

use test_case::test_case;

#[test_case(b"", b"")]
#[test_case(b"blah", b"blah")]
fn transformation_tests(input: &[u8], expected: &[u8]) {
    let mut id = Indenter::from(Vec::new());
    id.write_all(input).unwrap();
    let actual = &id.unwrap()[..];
    assert_eq!(actual, expected);
}
