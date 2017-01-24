pub struct Sticky<T: io::Write> {
    writer: T,
    count: u64,
    error: Option<Error>,
}

impl io::Write for Sticky {
    fn write(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error {
            Some(e) => return e,
            None => {
                match self.writer.write(buf) {
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
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<T: io::Read> ops::Deref for Sticky<T> {
    type Target = T;

    /// The deref function allows access to the wrapped io::Write.
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
