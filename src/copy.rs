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
    Growable(Cursor<Vec<u8>>),
    Sized(Cursor<&'a mut [u8]>),
}

impl<'a> StreamWriteBuffer<'a> {
    pub fn new_growable(size: Option<usize>) -> Self {
        match size {
            Some(sz) => StreamWriteBuffer::Growable(Cursor::new(vec![0; sz])),
            None => StreamWriteBuffer::Growable(Cursor::new(vec![])),
        }
    }

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
