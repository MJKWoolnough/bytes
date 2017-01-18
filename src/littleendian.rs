use std::io;
use std::ops;
use EndianRead;
use EndianWrite;

/// The Read struct wraps a reader to provide little endian reading functions, satisfying the
/// EndianRead trait.
pub struct Read<T: io::Read> {
    reader: T,
}

impl<T: io::Read> Read<T> {
    /// Constructs a new Little Endian `Read<T>`, wrapping the given io::Read.
    pub fn new(data: T) -> Read<T> {
        Read { reader: data }
    }
}

impl<T: io::Read> io::Read for Read<T> {
    /// The read function forwards the call to the embedded io::Read.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<T: io::Read> EndianRead for Read<T> {
    /// Read a single unsigned 16-bit integer from the io::Read type in Little Endian format.
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => Ok((buffer[1] as u16) << 8 | (buffer[0] as u16)),
            Err(e) => Err(e),
        }
    }
    /// Read a single unsigned 32-bit integer from the io::Read type in Little Endian format.
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                Ok((buffer[3] as u32) << 24 | (buffer[2] as u32) << 16 | (buffer[1] as u32) << 8 |
                   (buffer[0] as u32))
            } 
            Err(e) => Err(e),
        }
    }
    /// Read a single unsigned 64-bit integer from the io::Read type in Little Endian format.
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => {
                Ok((buffer[7] as u64) << 56 | (buffer[6] as u64) << 48 |
                   (buffer[5] as u64) << 40 | (buffer[4] as u64) << 32 |
                   (buffer[3] as u64) << 24 | (buffer[2] as u64) << 16 |
                   (buffer[1] as u64) << 8 | (buffer[0] as u64))
            }
            Err(e) => Err(e),
        }
    }
}

impl<T: io::Read> ops::Deref for Read<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Read.
    fn deref(&self) -> &T {
        &self.reader
    }
}

/// The Write struct wraps a type that implements the io::Write to provide Little Endian reading functions, satisfying the EndianRead trait.
pub struct Write<T: io::Write> {
    writer: T,
}

impl<T: io::Write> Write<T> {
    /// Constructs a new Little Endian `Write<T>`, wrapping the given io::Write.
    pub fn new(data: T) -> Write<T> {
        Write { writer: data }
    }
}

impl<T: io::Write> io::Write for Write<T> {
    /// The write function forwards the call to the embedded io::Write.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }
    /// The flush function forwards the call to the embedded io::Write.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<T: io::Write> EndianWrite for Write<T> {
    /// Write a single unsigned 16-bit integer to the io::Write type in Little Endian format.
    fn write_u16(&mut self, v: u16) -> io::Result<()> {
        let buffer = [v as u8, (v >> 8) as u8];
        self.writer.write_all(&buffer)
    }
    /// Write a single unsigned 32-bit integer to the io::Write type in Little Endian format.
    fn write_u32(&mut self, v: u32) -> io::Result<()> {
        let buffer = [v as u8, (v >> 8) as u8, (v >> 16) as u8, (v >> 24) as u8];
        self.writer.write_all(&buffer)
    }
    /// Write a single unsigned 64-bit integer to the io::Write type in Little Endian format.
    fn write_u64(&mut self, v: u64) -> io::Result<()> {
        let buffer = [v as u8,
                      (v >> 8) as u8,
                      (v >> 16) as u8,
                      (v >> 24) as u8,
                      (v >> 32) as u8,
                      (v >> 40) as u8,
                      (v >> 48) as u8,
                      (v >> 56) as u8];
        self.writer.write_all(&buffer)
    }
}

impl<T: io::Write> ops::Deref for Write<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Write.
    fn deref(&self) -> &T {
        &self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::super::buffer::Buffer;
    use super::super::EndianRead;
    use super::super::EndianWrite;

    #[test]
    fn read_test() {
        let mut buffer = super::Read::new(Buffer::new(vec![13, 12, 11, 10, 13, 12, 8, 7, 6, 5,
                                                           4, 3, 2, 1, 65]));

        assert_eq!(buffer.read_u32().expect("unexpected error"), 168496141);
        assert_eq!(buffer.read_u16().expect("unexpected error"), 3085);
        assert_eq!(buffer.read_u64().expect("unexpected error"),
                   72623859790382856);
        assert_eq!(buffer.read_u8().expect("unexpected error"), 65);
    }
    #[test]
    fn write_test() {
        let mut buffer = super::Write::new(Buffer::new(Vec::new()));

        buffer.write_u32(168496141).expect("unexpected error");
        buffer.write_u16(3085).expect("unexpected error");
        buffer.write_u64(72623859790382856).expect("unexpected error");
        buffer.write_u8(65).expect("unexpected error");
        assert_eq!(**buffer,
                   vec![13, 12, 11, 10, 13, 12, 8, 7, 6, 5, 4, 3, 2, 1, 65]);
    }

}
