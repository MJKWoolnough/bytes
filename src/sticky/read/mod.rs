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
