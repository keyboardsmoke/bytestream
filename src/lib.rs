pub mod reader;
pub mod writer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByteStreamError
{
    #[error("IO Error")]
    IOError(#[from] std::io::Error),

    #[error("FromUtf8Error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("UTF-16 encoding error")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),

    #[error("Core message")]
    Message(String),
}

#[cfg(test)]
mod tests
{
    use crate::{ByteStreamError, reader::{self, Reader}, writer::{self, Writer}};

    #[test]
    fn test_string_reader_utf8_null() -> Result<(), ByteStreamError>
    {
        let data: [u8; 3] = ['h' as u8, 'i' as u8, 0];
        let mut r = reader::from_slice(data.as_slice());
        let s = r.read_utf8_string_terminated()?;
        assert_eq!(s, "hi");
        Ok(())
    }

    #[test]
    fn test_string_reader_writer_utf8() -> Result<(), ByteStreamError>
    {
        let mut stream = writer::create();
        stream.write_utf8_string_terminated(String::from("hi"));
        stream.write_utf8_string_terminated(String::from("bye"));

        let mut r = reader::from_vec(stream.get());
        let a = r.read_utf8_string_terminated()?;
        let b = r.read_utf8_string_terminated()?;

        assert_eq!(a, "hi");
        assert_eq!(b, "bye");
        Ok(())
    }

    #[test]
    fn test_string_reader_utf8_null_twice() -> Result<(), ByteStreamError>
    {
        let data: [u8; 7] = ['h' as u8, 'i' as u8, 0, 'b' as u8, 'y' as u8, 'e' as u8, 0];
        let mut r = reader::from_slice(data.as_slice());
        let s = r.read_utf8_string_terminated()?;
        assert_eq!(s, "hi");
        let s = r.read_utf8_string_terminated()?;
        assert_eq!(s, "bye");
        Ok(())
    }

    #[test]
    fn test_string_reader_utf16_null() -> Result<(), ByteStreamError>
    {
        let data: [u16; 3] = ['h' as u16, 'i' as u16, 0];
        let byteslice: &[u8] = unsafe { std::slice::from_raw_parts(data.as_slice().as_ptr() as *const u8, data.len() * std::mem::size_of::<u16>()) };
        let mut r = reader::from_slice(byteslice);
        let s = r.read_utf16_string_terminated()?;
        assert_eq!(s, "hi");
        Ok(())
    }

    #[test]
    fn test_string_reader_writer_utf16() -> Result<(), ByteStreamError>
    {
        let mut stream = writer::create();
        stream.write_utf16_string_terminated(String::from("hi"));
        stream.write_utf16_string_terminated(String::from("bye"));

        let mut r = reader::from_vec(stream.get());
        let a = r.read_utf16_string_terminated()?;
        let b = r.read_utf16_string_terminated()?;

        assert_eq!(a, "hi");
        assert_eq!(b, "bye");
        Ok(())
    }

    #[test]
    fn test_string_reader_utf16_null_twice() -> Result<(), ByteStreamError>
    {
        let data: [u16; 7] = ['h' as u16, 'i' as u16, 0, 'b' as u16, 'y' as u16, 'e' as u16, 0];
        let byteslice: &[u8] = unsafe { std::slice::from_raw_parts(data.as_slice().as_ptr() as *const u8, data.len() * std::mem::size_of::<u16>()) };
        let mut r = reader::from_slice(byteslice);
        let s = r.read_utf16_string_terminated()?;
        assert_eq!(s, "hi");
        let s = r.read_utf16_string_terminated()?;
        assert_eq!(s, "bye");
        Ok(())
    }
}