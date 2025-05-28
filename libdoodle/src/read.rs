use std::{
    ffi::CString,
    io::{self, BufRead, Read, Seek},
};

use crate::error::GenericResult;

/// An extension for [std::io::Read] that does the allocations for you
pub trait ReadExt: Read {
    /// Read a fixed number of bytes specified at compile time into a fixed-size array
    fn read_const_num_of_bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Read a specific amount of bytes
    fn read_num_of_bytes(&mut self, num: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; num];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Read the entire contents into a vec
    fn read_to_vec(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = vec![];
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Read a little endian u32
    fn read_u32_le(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.read_const_num_of_bytes()?))
    }
}

impl<T: Read> ReadExt for T {}

/// An extension for [std::io::BufRead] that does the allocations for you
pub trait BufReadExt: BufRead {
    /// Read a null-terminated C-style string
    fn read_cstring(&mut self) -> GenericResult<CString> {
        let mut buf = vec![];
        self.read_until(0, &mut buf)?;
        Ok(CString::from_vec_with_nul(buf)?)
    }
}

impl<T: BufRead> BufReadExt for T {}

/// An extension for [std::io::BufRead] + [std::io::Seek] that does the allocations for you
pub trait BufReadSeekExt: BufRead + Seek {
    /// Read a string with a maximum length that might be terminated early with a null byte.
    /// Always makes sure the cursor ends up at `current + length` as if `length` bytes were always read
    fn read_null_padded_string(&mut self, length: usize) -> GenericResult<CString> {
        let mut buf = vec![];
        let read = self.read_until(0, &mut buf)?;
        // Ensure there's always a null byte
        if !buf.ends_with(&[0]) {
            buf.push(0);
        }
        dbg!(&buf);
        self.seek_relative((length - read) as i64)?;
        Ok(CString::from_vec_with_nul(buf)?)
    }
}

impl<T: BufRead + Seek> BufReadSeekExt for T {}
