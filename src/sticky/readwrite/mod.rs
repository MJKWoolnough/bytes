use std::io;
use std::ops;

pub struct Sticky<T: io::Read + io::Write> {
    readwriter: T,
    read_count: usize,
    write_count: usize,
    error: Option<io::Error>,
}

impl<T: io::Read + io::Write> io::Read for Sticky<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error {
            Some(_) => Ok(buf.len()),
            None => {
                match self.readwriter.read(buf) {
                    Ok(c) => {
                        self.read_count += c;
                        Ok(c)
                    }
                    Err(e) => {
                        self.error = Some(e);
                        Ok(buf.len())
                    }
                }
            }
        }
    }
}

impl<T: io::Read + io::Write> io::Write for Sticky<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.error {
            Some(_) => Ok(buf.len()),
            None => {
                match self.readwriter.write(buf) {
                    Ok(c) => {
                        self.write_count += c;
                        Ok(c)
                    }
                    Err(e) => {
                        self.error = Some(e);
                        Ok(buf.len())
                    }
                }
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        self.readwriter.flush()
    }
}


impl<T: io::Read + io::Write> ops::Deref for Sticky<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Read.
    fn deref(&self) -> &T {
        &self.readwriter
    }
}

impl<T: io::Read + io::Write> Sticky<T> {
    pub fn new(readwrite: T) -> Sticky<T> {
        Sticky {
            readwriter: readwrite,
            read_count: 0,
            write_count: 0,
            error: None,
        }
    }
    pub fn error(self) -> Option<io::Error> {
        self.error
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn read_count(&self) -> usize {
        self.read_count
    }
    pub fn write_count(&self) -> usize {
        self.write_count
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sticky_read_test() {}
}
