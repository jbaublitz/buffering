use std::io::{self,Cursor,Read,Write};

pub struct StreamReadBuffer<T>(Cursor<T>);

impl<T> StreamReadBuffer<T> where T: AsRef<[u8]> {
    pub fn new(buf: T) -> Self {
        StreamReadBuffer(Cursor::new(buf))
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

pub enum StreamWriteBuffer<'a> {
    Growable(Cursor<Vec<u8>>, usize),
    Sized(Cursor<&'a mut [u8]>, usize),
}

impl<'a> StreamWriteBuffer<'a> {
    pub fn new_growable(size: Option<usize>) -> Self {
        match size {
            Some(sz) => StreamWriteBuffer::Growable(Cursor::new(vec![0; sz]), 0),
            None => StreamWriteBuffer::Growable(Cursor::new(vec![]), 0),
        }
    }

    pub fn new_sized(buf: &'a mut [u8]) -> Self {
        StreamWriteBuffer::Sized(Cursor::new(buf), 0)
    }

    pub unsafe fn set_bytes_written(&mut self, size: usize) {
        match *self {
            StreamWriteBuffer::Growable(_, ref mut len) => {
                *len += size;
            },
            StreamWriteBuffer::Sized(_, ref mut len) => {
                *len += size;
            },
        }
    }
}

impl<'a> Write for StreamWriteBuffer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(match *self {
            StreamWriteBuffer::Growable(ref mut c, ref mut len) => {
                let bytes_written = c.write(buf)?;
                *len += bytes_written;
                bytes_written
            },
            StreamWriteBuffer::Sized(ref mut c, ref mut len) => {
                let bytes_written = c.write(buf)?;
                *len += bytes_written;
                bytes_written
            },
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> AsRef<[u8]> for StreamWriteBuffer<'a> {
    fn as_ref(&self) -> &[u8] {
        match *self {
            StreamWriteBuffer::Growable(ref c, ref len) => &c.get_ref()[0..*len],
            StreamWriteBuffer::Sized(ref c, ref len) => &c.get_ref()[0..*len],
        }
    }
}

impl<'a> AsMut<[u8]> for StreamWriteBuffer<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        match *self {
            StreamWriteBuffer::Growable(ref mut c, _) => c.get_mut(),
            StreamWriteBuffer::Sized(ref mut c, _) => c.get_mut(),
        }
    }
}
