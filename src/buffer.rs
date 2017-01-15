use std::io;
use std::ops;
use std::vec;

struct Buffer {
    data: Vec<u8>,
}

impl io:Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let l = self.data.len();
        let m = buf.len();
        let n = if l > m {
            m
        } else {
            l
        };
        for (n, b) in self.data.drain(0..n).enumerate(){
            buf[n] = b
        }
        Ok(n)
    }
}


impl io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.reserve(buf.len());
        self.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl ops::Defer for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.data
    }
}
