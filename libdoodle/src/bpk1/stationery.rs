use serde::Serialize;
use std::{error::Error, fmt::Display, io::Cursor};

use super::{BPK1Block, BPK1File, BlocksHashMap};
use crate::{bits::PickBit, error::GenericResult, read::BufReadSeekExt};

#[derive(Debug, Serialize)]
pub struct Stationery {
    pub name: String,
    pub background_2d: Vec<u8>,
    pub background_3d: Vec<u8>,
    pub mask: Vec<Vec<u8>>,
    pub blocks: BlocksHashMap,
}

#[derive(Debug, Serialize)]
pub enum StationeryDeserializeError {
    MissingHeader,
    MissingBothBackgrounds,
    Missing3DBackground,
    TooManyBackgrounds,
    MissingMask,
}

impl Display for StationeryDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StationeryDeserializeError::*;
        match self {
            MissingHeader => write!(f, "Missing header block"),
            MissingBothBackgrounds => write!(f, "Missing both stationery background blocks"),
            Missing3DBackground => write!(f, "Missing 3D stationery background block"),
            TooManyBackgrounds => write!(f, "Too many background blocks"),
            MissingMask => write!(f, "Missing mask block"),
        }
    }
}

impl Error for StationeryDeserializeError {}

impl BPK1File for Stationery {
    fn new_from_bpk1_blocks(blocks: Vec<BPK1Block>) -> GenericResult<Self> {
        let mut name = None;
        let mut background_2d = None;
        let mut background_3d = None;
        let mut mask = None;

        for block in &blocks {
            // Apparently you can't cleanly match against CString; so I'll just use a byte string. Essentially identical
            match block.name.to_bytes() {
                b"STAHED1" => {
                    let mut cursor = Cursor::new(&block.data);
                    name = Some(cursor.read_null_padded_string(0x80)?.into_string()?);
                }
                b"STBARD1" => {
                    if background_2d.is_none() {
                        background_2d = Some(block.data.to_owned())
                    } else if background_3d.is_none() {
                        background_3d = Some(block.data.to_owned())
                    } else {
                        Err(StationeryDeserializeError::TooManyBackgrounds)?
                    }
                }
                b"STMASK1" => mask = Some(read_mask(&block.data)),
                _ => {}
            }
        }

        Ok(Stationery {
            name: name.ok_or(StationeryDeserializeError::MissingHeader)?,
            background_2d: background_2d
                .ok_or(StationeryDeserializeError::MissingBothBackgrounds)?,
            background_3d: background_3d.ok_or(StationeryDeserializeError::Missing3DBackground)?,
            mask: mask.ok_or(StationeryDeserializeError::MissingMask)?,
            blocks: BlocksHashMap::new_from_bpk1_blocks(blocks)?,
        })
    }
}

// Implements a Z order curve for 0..64
fn z_order_curve(i: usize) -> (usize, usize) {
    (
        (i & 1) | ((i & 0b100) >> 1) | ((i & 0b10000) >> 2),
        ((i & 2) >> 1) | ((i & 0b1000) >> 2) | ((i & 0b100000) >> 3),
    )
}

const BLOCK_SIZE: usize = 8;

// Passing the slice by reference because it's quite large.
// FIXME: Too many magic numbers, too few constants
// I implemented this quite crudely because I was tired -CenTdemeern1
fn read_mask(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut image = vec![vec![0u8; 256]; 256];
    // .cloned() is cloning a u8. This operation is free
    for (l4_index, byte) in bytes.iter().take(32768).cloned().enumerate() {
        let block_index = l4_index / 32;
        let px_index_within_block = l4_index % 128;
        let block_x = block_index % 32;
        let block_y = block_index / 32;
        let (x, y) = z_order_curve(px_index_within_block * 2);
        let x = block_x * BLOCK_SIZE + x;
        let y = block_y * BLOCK_SIZE + y;
        image[y][x] = byte.pick_bits(0..=3);
        image[y][x + 1] = byte.pick_bits(4..=7);
    }
    image
}
