use std::io::{Read, Result, Write};
pub struct ReadStats<R: Read> {
    data: R,
    reads: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(data: R) -> ReadStats<R> {
        ReadStats {
            data,
            reads: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let curr_bytes = self.data.read(buf)?;
        self.reads += 1;
        self.bytes += curr_bytes;

        Ok(curr_bytes)
    }
}

pub struct WriteStats<W> {
    data: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(data: W) -> WriteStats<W> {
        WriteStats {
            data,
            writes: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let curr_bytes = self.data.write(buf)?;
        self.writes += 1;
        self.bytes += curr_bytes;

        Ok(curr_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
