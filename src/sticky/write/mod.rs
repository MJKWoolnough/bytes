use std::io;
use std::ops;

pub struct Sticky<T: io::Write> {
    writer: T,
    count: usize,
    error: Option<io::Error>,
}

impl<T: io::Write> io::Write for Sticky<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.error {
            Some(_) => Ok(buf.len()),
            None => {
                match self.writer.write(buf) {
                    Ok(c) => {
                        self.count += c;
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
        self.writer.flush()
    }
}

impl<T: io::Write> ops::Deref for Sticky<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Write.
    fn deref(&self) -> &T {
        &self.writer
    }
}

impl<T: io::Write> Sticky<T> {
    pub fn new(writer: T) -> Sticky<T> {
        Sticky {
            writer: writer,
            count: 0,
            error: None,
        }
    }
    pub fn error(self) -> Option<io::Error> {
        self.error
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn count(self) -> usize {
        self.count
    }
    pub fn result(self) -> io::Result<usize> {
        match self.error {
            Some(e) => Err(e),
            None => Ok(self.count),
        }
    }
}
