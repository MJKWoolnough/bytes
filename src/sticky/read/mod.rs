use std::io;
use std::ops;

pub struct Sticky<T: io::Read> {
    reader: T,
    count: usize,
    error: Option<io::Error>,
}

impl<T: io::Read> io::Read for Sticky<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error {
            Some(_) => Ok(buf.len()),
            None => {
                match self.reader.read(buf) {
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
}

impl<T: io::Read> ops::Deref for Sticky<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Read.
    fn deref(&self) -> &T {
        &self.reader
    }
}

impl<T: io::Read> Sticky<T> {
    pub fn new(reader: T) -> Sticky<T> {
        Sticky {
            reader: reader,
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
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn result(self) -> io::Result<usize> {
        match self.error {
            Some(e) => Err(e),
            None => Ok(self.count),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn sticky_read_test() {}
}
