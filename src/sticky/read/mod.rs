pub struct Sticky<T: io::Read> {
    reader: T,
    count: u64,
    error: Option<Error>,
}

impl io::Read for Sticky {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error.is_some() {
            Some(e) => return e,
            None => {
                match self.reader.read(buf) {
                    Ok(c) => {
                        self.count += c;
                        Ok(c)
                    }
                    Err(e) => {
                        self.error = Self(e);
                        Err(e)
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
    fn error(&mut self) -> Option<Error> {
        self.error
    }
    fn has_error(&mut self) -> bool {
        self.error.is_some()
    }
    fn count(&mut self) -> u64 {
        self.count
    }
}
