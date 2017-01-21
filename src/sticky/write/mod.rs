pub struct Sticky<T: io::Write> {
    writer: T,
    count: u64,
    error: Option<Error>,
}

impl io::Write for Sticky {
    fn write(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.error.is_some() {
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
