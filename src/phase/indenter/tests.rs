use super::Indenter;
use std::io::Write;

use test_case::test_case;

#[test_case(&String::new(), &String::new())]
#[test_case("blah", "blah")]
#[test_case("foo\nbar\n", "| foo\n| bar\n")]
fn transformation_tests(input: &str, expected: &str) {
    let mut id = Indenter::from(Vec::new());
    id.write_all(input.as_bytes()).unwrap();
    let outvec = id.unwrap();
    let actual = std::str::from_utf8(&outvec).unwrap();
    assert_eq!(actual, expected);
}
