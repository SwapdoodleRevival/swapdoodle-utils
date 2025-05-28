pub mod letter;
pub mod stationery;

use std::{
    collections::HashMap,
    error::Error,
    ffi::CString,
    fmt::Display,
    io::{self, BufRead, Cursor, Seek, SeekFrom},
};

use crate::{
    error::GenericResult,
    lzss::decompress_from_slice,
    read::{BufReadSeekExt, ReadExt},
};

pub struct BPK1Block {
    pub name: CString,
    pub data: Vec<u8>,
}

fn has_bpk1_magic(reader: &[u8]) -> bool {
    reader.get(0..4).is_some_and(|magic| magic == *b"BPK1")
}

#[derive(Debug, Clone, Copy)]
pub enum BPK1Error {
    BadMagic,
}

impl Display for BPK1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad BPK1 magic")
    }
}

impl Error for BPK1Error {}

pub trait BPK1File
where
    Self: Sized,
{
    fn new_from_bpk1_bytes(data: &[u8]) -> GenericResult<Self> {
        let mut reader: Box<dyn CursorTrait> = if has_bpk1_magic(data) {
            Box::new(Cursor::new(data))
        } else {
            let decompressed = decompress_from_slice(data)?;
            if !has_bpk1_magic(&decompressed) {
                Err(BPK1Error::BadMagic)?;
            }
            Box::new(Cursor::new(decompressed))
        };

        reader.seek_relative(4)?;
        let num_blocks = reader.read_u32_le()?;
        reader.seek_relative(0x38)?;

        struct BlockHeader {
            offset: u32,
            size: u32,
            checksum: u32,
            name: CString,
        }

        let mut blocks = Vec::with_capacity(num_blocks as usize);

        for _ in 0..num_blocks {
            blocks.push(BlockHeader {
                offset: reader.read_u32_le()?,
                size: reader.read_u32_le()?,
                checksum: reader.read_u32_le()?,
                name: reader.read_null_padded_string(8)?,
            })
        }

        // Turn the headers into contentful blocks
        // Doing this *after* reading the headers since this involves seeking
        let blocks = blocks
            .into_iter()
            .map(|head| {
                let BlockHeader {
                    offset,
                    size,
                    checksum: _,
                    name,
                } = head;

                reader.seek(SeekFrom::Start(offset as u64))?;
                println!(
                    "Reading {} at offset {offset} with size {size}",
                    name.to_string_lossy()
                );

                Ok(BPK1Block {
                    name,
                    data: reader.read_num_of_bytes(size as usize)?,
                })
            })
            .collect::<io::Result<Vec<BPK1Block>>>()?; // Collect into a Result<Vec> from an Iterator<Item = Result> to short circuit

        Self::new_from_bpk1_blocks(blocks)
    }

    fn new_from_bpk1_blocks(blocks: Vec<BPK1Block>) -> GenericResult<Self>;
}

trait CursorTrait: BufRead + Seek {}
impl<T: AsRef<[u8]>> CursorTrait for Cursor<T> {}

pub type BlocksHashMap = HashMap<String, Vec<Vec<u8>>>;

impl BPK1File for BlocksHashMap {
    fn new_from_bpk1_blocks(blocks: Vec<BPK1Block>) -> GenericResult<Self> {
        let mut map = Self::new();
        for block in blocks {
            map.entry(block.name.into_string()?)
                .or_insert_with(Vec::new)
                .push(block.data);
        }
        Ok(map)
    }
}
