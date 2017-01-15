use std::io;
use std::ops;
use EndianRead;
use EndianWrite;

struct Read<T: io::Read> {
    reader: T,
}

impl<T: io::Read> io::Read for Read<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<T: io::Read> EndianRead for Read<T> {
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => Ok((buffer[1] as u16) | (buffer[0] as u16) << 8),
            Err(e) => Err(e),
        }
    }
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                Ok((buffer[3] as u32) | (buffer[2] as u32) << 8 | (buffer[1] as u32) << 16 |
                   (buffer[0] as u32) << 24)
            }
            Err(e) => Err(e),
        }
    }
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                Ok((buffer[7] as u64) | (buffer[6] as u64) << 8 | (buffer[5] as u64) << 16 |
                   (buffer[4] as u64) << 24 | (buffer[3] as u64) << 32 |
                   (buffer[2] as u64) << 40 |
                   (buffer[1] as u64) << 48 | (buffer[0] as u64) << 56)
            }
            Err(e) => Err(e),
        }
    }
}

impl<T: io::Read> ops::Deref for Write<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.reader
    }
}


struct Write<T: io::Write> {
    writer: T,
}

impl<T: io::Write> io::Write for Write<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<T: io::Write> EndianWrite for Write<T> {
    fn write_u16(&mut self, v: u16) -> io::Result<()> {
        let buffer = [(v >> 8) as u8, v as u8];
        self.writer.write_all(&buffer)
    }
    fn write_u32(&mut self, v: u32) -> io::Result<()> {
        let buffer = [(v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8, v as u8];
        self.writer.write_all(&buffer)
    }
    fn write_u64(&mut self, v: u64) -> io::Result<()> {
        let buffer = [(v >> 56) as u8,
                      (v >> 48) as u8,
                      (v >> 40) as u8,
                      (v >> 32) as u8,
                      (v >> 24) as u8,
                      (v >> 16) as u8,
                      (v >> 8) as u8,
                      v as u8];
        self.writer.write_all(&buffer)
    }
}

impl<T: io::Write> ops::Deref for Write<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.writer
    }
}
