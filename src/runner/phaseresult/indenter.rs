use std::io::{Result, Write};

pub struct Indenter<W> {
    f: W,
    linestart: bool,
}

impl<W> From<W> for Indenter<W> {
    fn from(f: W) -> Indenter<W> {
        Indenter { f, linestart: true }
    }
}

impl<W: Write> Write for Indenter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        let mut chunks = buf.split(|&c| c == b'\n').fuse();
        let mut ochunk = chunks.next();

        for nextchunk in chunks {
            let chunk = ochunk.expect("Indenter::write internal loop invariant failed");
            written += self.write_eol_chunk(chunk)?;
            ochunk = Some(nextchunk);
        }

        if let Some(chunk) = ochunk {
            written += self.write_trailing_chunk(chunk)?;
        }

        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.f.flush()
    }
}

impl<W: Write> Indenter<W> {
    fn write_eol_chunk(&mut self, chunk: &[u8]) -> Result<usize> {
        self._write_chunk(chunk, true)
    }

    fn write_trailing_chunk(&mut self, chunk: &[u8]) -> Result<usize> {
        if !chunk.is_empty() {
            self._write_chunk(chunk, false)
        } else {
            Ok(0)
        }
    }

    fn _write_chunk(&mut self, chunk: &[u8], newline: bool) -> Result<usize> {
        if self.linestart {
            self.f.write_all("| ".as_bytes())?;
        }
        self.linestart = newline;

        self.f.write_all(chunk)?;
        if newline {
            self.f.write_all(&[b'\n'])?;
        }

        Ok(chunk.len() + if newline { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::Indenter;
    use std::io::Write;

    use test_case::test_case;

    impl<W> Indenter<W> {
        fn unwrap(self) -> W {
            self.f
        }
    }

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
}
