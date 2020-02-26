use std::io::{Result, Write};

pub struct Indenter<W>(W);

impl<W> From<W> for Indenter<W> {
    fn from(f: W) -> Indenter<W> {
        Indenter(f)
    }
}

impl<W: Write> Write for Indenter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut total = 0;
        let mut sep: &[u8] = &[];
        for chunk in buf.split(|&c| c == b'\n') {
            self.0.write_all(sep)?;
            self.0.write_all(chunk)?;
            total += chunk.len();
            if sep.len() > 0 {
                total += 1;
            }
            sep = &[b'\n', b'|', b' '];
        }
        Ok(total)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
