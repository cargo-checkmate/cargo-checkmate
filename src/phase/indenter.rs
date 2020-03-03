#[cfg(test)]
mod tests;
mod writecounter;

use std::io::{Result, Write};

pub struct Indenter<W>(W);

impl<W> From<W> for Indenter<W> {
    fn from(f: W) -> Indenter<W> {
        Indenter(f)
    }
}

impl<W: Write> Write for Indenter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        use self::writecounter::WriteCounter;

        let mut wc = WriteCounter::new(&mut self.0);
        let mut first = true;

        for chunk in buf.split(|&c| c == b'\n') {
            if first {
                first = false;
            } else {
                wc.write_counted("\n")?;
                wc.write_uncounted("| ")?;
            }

            wc.write_counted(chunk)?;
        }

        Ok(wc.final_tally())
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

#[cfg(test)]
impl<W> Indenter<W> {
    fn unwrap(self) -> W {
        self.0
    }
}
