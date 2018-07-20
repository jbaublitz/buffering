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
