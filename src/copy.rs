use std::io::{self,Cursor,Read,Write};

/// A stream reader that will allow piece-by-piece reading of a buffer
pub struct StreamReadBuffer<T>(Cursor<T>);

impl<T> StreamReadBuffer<T> where T: AsRef<[u8]> {
    /// Create a new reader with an underlying data type that can be expresssed as a byte slice
    pub fn new(buf: T) -> Self {
        StreamReadBuffer(Cursor::new(buf))
    }

    /// Check whether the stream has reached the end of the underlying buffer
    pub fn at_end(&self) -> bool {
        match self {
            StreamReadBuffer(c) => {
                c.position() == c.get_ref().as_ref().len() as u64
            }
        }
    }
}

impl<T> Read for StreamReadBuffer<T> where T: AsRef<[u8]> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<T> AsRef<[u8]> for StreamReadBuffer<T> where T: AsRef<[u8]> {
    fn as_ref(&self) -> &[u8] {
        self.0.get_ref().as_ref()
    }
}

/// A stream writer that will allow piece-by-piece writing of to a buffer
pub enum StreamWriteBuffer<'a> {
    /// A wrapper around a byte vector-based cursor
    Growable(Cursor<Vec<u8>>),
    /// A wrapper around a mutable slice-based cursor
    Sized(Cursor<&'a mut [u8]>),
}

impl<'a> StreamWriteBuffer<'a> {
    /// Create a new vector-based stream writer that can grow when written past buffer boundaries
    pub fn new_growable(size: Option<usize>) -> Self {
        match size {
            Some(sz) => StreamWriteBuffer::Growable(Cursor::new(vec![0; sz])),
            None => StreamWriteBuffer::Growable(Cursor::new(vec![])),
        }
    }

    /// Create a new slice-based stream writer that will error when written past buffer boundaries
    pub fn new_sized(buf: &'a mut [u8]) -> Self {
        StreamWriteBuffer::Sized(Cursor::new(buf))
    }
}

impl<'a> Write for StreamWriteBuffer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            StreamWriteBuffer::Growable(ref mut c) => c.write(buf),
            StreamWriteBuffer::Sized(ref mut c) => c.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> AsRef<[u8]> for StreamWriteBuffer<'a> {
    fn as_ref(&self) -> &[u8] {
        match *self {
            StreamWriteBuffer::Growable(ref c) => &c.get_ref()[0..c.position() as usize],
            StreamWriteBuffer::Sized(ref c) => &c.get_ref()[0..c.position() as usize],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at_end_method() {
        let mut b = StreamReadBuffer::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let first_second_read = &mut [0u8; 4];
        b.read(first_second_read).unwrap();
        assert_eq!(b.at_end(), false);
        b.read(first_second_read).unwrap();
        assert_eq!(b.at_end(), false);
        let last_read = &mut [0u8; 2];
        b.read(last_read).unwrap();
        assert_eq!(b.at_end(), true);
    }
}
