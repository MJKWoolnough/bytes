use std::io;
use std::mem;

pub mod littleendian;
pub mod bigendian;
pub mod buffer;

/// The EndianRead trait extends the io::Read trait to include readers for various number types.
pub trait EndianRead: io::Read {
    /// Read a single unsigned 8-bit integer from the reader.
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0];
        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(e) => Err(e),
        }
    }
    /// Read a single unsigned 16-bit integer from the reader.
    fn read_u16(&mut self) -> io::Result<u16>;
    /// Read a single unsigned 32-bit integer from the reader.
    fn read_u32(&mut self) -> io::Result<u32>;
    /// Read a single unsigned 64-bit integer from the reader.
    fn read_u64(&mut self) -> io::Result<u64>;
    /// Read a single signed 8-bit integer from the reader.
    fn read_i8(&mut self) -> io::Result<i8> {
        match self.read_u8() {
            Ok(v) => Ok(v as i8),
            Err(e) => Err(e),
        }
    }
    /// Read a single signed 16-bit integer from the reader.
    fn read_i16(&mut self) -> io::Result<i16> {
        match self.read_u16() {
            Ok(v) => Ok(v as i16),
            Err(e) => Err(e),
        }
    }
    /// Read a single signed 32-bit integer from the reader.
    fn read_i32(&mut self) -> io::Result<i32> {
        match self.read_u32() {
            Ok(v) => Ok(v as i32),
            Err(e) => Err(e),
        }
    }
    /// Read a single signed 64-bit integer from the reader.
    fn read_i64(&mut self) -> io::Result<i64> {
        match self.read_u64() {
            Ok(v) => Ok(v as i64),
            Err(e) => Err(e),
        }
    }
    /// Read a single 32-bit floating point number from the reader.
    fn read_f32(&mut self) -> io::Result<f32> {
        match self.read_u32() {
            Ok(v) => Ok(unsafe { mem::transmute(v) }),
            Err(e) => Err(e),
        }
    }
    /// Read a single 64-bit floating point number from the reader.
    fn read_f64(&mut self) -> io::Result<f64> {
        match self.read_u64() {
            Ok(v) => Ok(unsafe { mem::transmute(v) }),
            Err(e) => Err(e),
        }
    }
}

/// The EndianWrite trait extends the io::Write trait to include writes for various number types.
pub trait EndianWrite: io::Write {
    /// Write a single unsigned 8-bit integer to the writer
    fn write_u8(&mut self, v: u8) -> io::Result<()> {
        let buffer = [v];
        self.write_all(&buffer)
    }
    /// Write a single unsigned 16-bit integer to the writer
    fn write_u16(&mut self, v: u16) -> io::Result<()>;
    /// Write a single unsigned 32-bit integer to the writer
    fn write_u32(&mut self, v: u32) -> io::Result<()>;
    /// Write a single unsigned 64-bit integer to the writer
    fn write_u64(&mut self, v: u64) -> io::Result<()>;
    /// Write a single signed 8-bit integer to the writer
    fn write_i8(&mut self, v: i8) -> io::Result<()> {
        self.write_u8(v as u8)
    }
    /// Write a single signed 16-bit integer to the writer
    fn write_i16(&mut self, v: i16) -> io::Result<()> {
        self.write_u16(v as u16)
    }
    /// Write a single signed 32-bit integer to the writer
    fn write_i32(&mut self, v: i32) -> io::Result<()> {
        self.write_u32(v as u32)
    }
    /// Write a single signed 64-bit integer to the writer
    fn write_i64(&mut self, v: i64) -> io::Result<()> {
        self.write_u64(v as u64)
    }
    /// Write a single 8-bit floating point number to the writer
    fn write_f32(&mut self, v: f32) -> io::Result<()> {
        self.write_u32(unsafe { mem::transmute(v) })
    }
    /// Write a single 32-bit floating point number to the writer
    fn write_f64(&mut self, v: f64) -> io::Result<()> {
        self.write_u64(unsafe { mem::transmute(v) })
    }
}
