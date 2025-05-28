use serde::Serialize;
use std::io::{Cursor, Error as IoError};
#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::{bits::PickBit, mii_data::name_from_bytes, read::ReadExt};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
pub struct Colors {
    pub colors: Vec<Color>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub id: u32,
    pub name: String,
}

impl Colors {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, IoError> {
        Self::try_from(bytes)
    }
}

fn full_rgb_value(nibble: u8) -> u8 {
    let full = nibble << 4 | 0x0F;
    if full == 0x0F {
        return 0;
    }
    full
}

impl Color {
    pub fn from_bytes(bytes: [u8; 0x4c]) -> Color {
        Color {
            r: full_rgb_value(bytes[5].pick_bits(4..=7)),
            g: full_rgb_value(bytes[5].pick_bits(0..=3)),
            b: full_rgb_value(bytes[4].pick_bits(4..=7)),
            a: full_rgb_value(bytes[4].pick_bits(0..=3)),
            id: u32::from_le_bytes(bytes[0..=3].try_into().unwrap()),
            name: name_from_bytes::<0x40>(bytes[6..=0x45].try_into().unwrap()),
        }
    }
}

impl TryFrom<&[u8]> for Colors {
    type Error = IoError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = Cursor::new(value);
        let num_colors = reader.read_u32_le()?;
        reader.read_u32_le()?;
        reader.set_position(0x10);

        let mut colors: Colors = Colors { colors: vec![] };

        for _ in 0..num_colors {
            colors
                .colors
                .push(Color::from_bytes(reader.read_const_num_of_bytes()?));
        }

        Ok(colors)
    }
}
