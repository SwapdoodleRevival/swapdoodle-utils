use serde::Serialize;
use std::io::{Cursor, Error as IoError};
#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::{bits::PickBit, read::ReadExt};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
pub struct Sheet {
    pub strokes: Vec<SheetStroke>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
pub struct SheetStroke {
    pub x: u8,
    pub y: u8,
    pub draw_line: bool,
    pub style_color: u8,
    pub style_3d: bool,
    pub style_bold: bool,
}

impl Sheet {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, IoError> {
        Self::try_from(bytes)
    }
}

impl SheetStroke {
    pub fn from_bytes(bytes: [u8; 4]) -> SheetStroke {
        SheetStroke {
            x: (bytes[2] & 0x0F) << 4 | bytes[1].pick_bits(4..=7),
            y: (bytes[1] & 0x0F) << 4 | bytes[0].pick_bits(4..=7),
            draw_line: bytes[2].pick_bit(6),
            style_color: bytes[3].pick_bits(0..=2),
            style_3d: bytes[2].pick_bit(5),
            style_bold: bytes[3].pick_bit(3),
        }
    }
}

impl TryFrom<&[u8]> for Sheet {
    type Error = IoError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = Cursor::new(value);
        reader.read_u32_le()?;
        let num_blocks = reader.read_u32_le()?;
        reader.set_position(0x40);

        let mut sheet: Sheet = Sheet { strokes: vec![] };

        for _ in 0..num_blocks {
            sheet
                .strokes
                .push(SheetStroke::from_bytes(reader.read_const_num_of_bytes()?));
        }

        Ok(sheet)
    }
}
