use std::io;
use std::ops;

struct Buffer {
    data: Vec<u8>,
}

impl io::Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let l = self.data.len();
        let m = buf.len();
        let n = if l > m { m } else { l };
        for (n, b) in self.data.drain(0..n).enumerate() {
            buf[n] = b
        }
        Ok(n)
    }
}


impl io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.reserve(buf.len());
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl ops::Deref for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::io::Write;
    use std::vec::Vec;

    #[test]
    fn buffer_read_test() {
        let mut buffer = super::Buffer { data: String::from("Hello, World!").into_bytes() };
        let mut buf = [0; 5];

        assert_eq!(buffer.data, String::from("Hello, World!").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 5);
        assert_eq!(buf, String::from("Hello").as_bytes());

        assert_eq!(buffer.data, String::from(", World!").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 5);
        assert_eq!(buf, String::from(", Wor").as_bytes());

        assert_eq!(buffer.data, String::from("ld!").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 3);
        assert_eq!(buf, String::from("ld!or").as_bytes());

        assert_eq!(buffer.data, String::from("").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 0);
        assert_eq!(buf, String::from("ld!or").as_bytes());
    }

    #[test]
    fn buffer_write_test() {
        let mut buffer = super::Buffer { data: Vec::new() };
        assert_eq!(buffer.write(String::from("J").as_bytes()).expect("unexpected error"),
                   1);
        assert_eq!(buffer.data, String::from("J").into_bytes());
        assert_eq!(buffer.write(String::from("ohn").as_bytes()).expect("unexpected error"),
                   3);
        assert_eq!(buffer.data, String::from("John").into_bytes());
        assert_eq!(buffer.write(String::from("ny").as_bytes()).expect("unexpected error"),
                   2);
        assert_eq!(buffer.data, String::from("Johnny").into_bytes());
        assert_eq!(buffer.write(String::from("").as_bytes()).expect("unexpected error"),
                   0);
        assert_eq!(buffer.data, String::from("Johnny").into_bytes());
    }

    #[test]
    fn buffer_read_write_test() {
        let mut buffer = super::Buffer { data: String::from("Beeping").into_bytes() };
        let mut buf = [0; 5];

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 5);
        assert_eq!(buf, String::from("Beepi").as_bytes());

        assert_eq!(buffer.write(String::from(" Hell").as_bytes()).expect("unexpected error"),
                   5);
        assert_eq!(buffer.data, String::from("ng Hell").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 5);
        assert_eq!(buf, String::from("ng He").as_bytes());

        assert_eq!(buffer.write(String::from("!").as_bytes()).expect("unexpected error"),
                   1);
        assert_eq!(buffer.data, String::from("ll!").into_bytes());

        assert_eq!(buffer.read(&mut buf).expect("unexpected error"), 3);
        assert_eq!(buf, String::from("ll!He").as_bytes());
    }
}
