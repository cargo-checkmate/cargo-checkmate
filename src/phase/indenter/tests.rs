use super::Indenter;
use std::io::Write;

use test_case::test_case;

#[test_case(&String::new(), &String::new())]
#[test_case("blah", "| blah")]
#[test_case("foo\nbar", "| foo\n| bar")]
#[test_case("foo\nbar\n", "| foo\n| bar\n")]
#[test_case("foo\n\nbar", "| foo\n| \n| bar")]
fn transformation_tests(input: &str, expected: &str) {
    for stride in 1..input.len() + 2 {
        let mut id = Indenter::from(Vec::new());

        for inchunk in input.as_bytes().chunks(stride) {
            id.write_all(inchunk).unwrap();
        }
        let outvec = id.unwrap();
        let actual = std::str::from_utf8(&outvec).unwrap();
        assert_eq!(actual, expected, "stride {}", stride);
    }
}
