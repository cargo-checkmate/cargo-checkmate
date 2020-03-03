use std::convert::AsRef;
use std::io::{Result, Write};

pub struct WriteCounter<'a, W> {
    w: &'a mut W,
    written: usize,
}

impl<'a, W: Write> WriteCounter<'a, W> {
    pub fn new(w: &'a mut W) -> WriteCounter<'a, W> {
        WriteCounter { w: w, written: 0 }
    }

    pub fn write_counted<B>(&mut self, buf: B) -> Result<()>
    where
        B: AsRef<[u8]>,
    {
        let bytes: &[u8] = buf.as_ref();
        self.w.write_all(bytes)?;
        self.written += bytes.len();
        Ok(())
    }

    pub fn write_uncounted<B>(&mut self, buf: B) -> Result<()>
    where
        B: AsRef<[u8]>,
    {
        self.w.write_all(buf.as_ref())
    }

    pub fn final_tally(self) -> usize {
        self.written
    }
}
