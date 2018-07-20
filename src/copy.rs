use std::io::{self,Cursor,Read,Write};

pub enum StreamReadBuffer<'a> {
    Growable(Cursor<Vec<u8>>),
    Static(Cursor<&'a [u8]>),
}

impl<'a> Read for StreamReadBuffer<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match *self {
            StreamReadBuffer::Growable(ref mut b) => b.read(buf),
            StreamReadBuffer::Static(ref mut b) => b.read(buf),
        }
    }
}

impl<'a> AsRef<[u8]> for StreamReadBuffer<'a> {
    fn as_ref(&self) -> &[u8] {
        match *self {
            StreamReadBuffer::Growable(ref b) => b.get_ref().as_slice(),
            StreamReadBuffer::Static(ref b) => b.get_ref(),
        }
    }
}

impl<'a> From<StreamWriteBuffer<'a>> for StreamReadBuffer<'a> {
    fn from(v: StreamWriteBuffer<'a>) -> Self {
        match v {
            StreamWriteBuffer::Growable(b) => StreamReadBuffer::Growable(b),
            StreamWriteBuffer::Static(b) => StreamReadBuffer::Static(Cursor::new(b.into_inner())),
        }
    }
}

pub enum StreamWriteBuffer<'a> {
    Growable(Cursor<Vec<u8>>),
    Static(Cursor<&'a mut [u8]>),
}

impl<'a> Write for StreamWriteBuffer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            StreamWriteBuffer::Growable(ref mut b) => b.write(buf),
            StreamWriteBuffer::Static(ref mut b) => b.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> AsMut<[u8]> for StreamWriteBuffer<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        match *self {
            StreamWriteBuffer::Growable(ref mut b) => b.get_mut().as_mut_slice(),
            StreamWriteBuffer::Static(ref mut b) => b.get_mut(),
        }
    }
}
