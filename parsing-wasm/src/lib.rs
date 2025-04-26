use std::collections::HashMap;

use parsing::{
    bpk1::{BPK1File, BlocksHashMap, letter::Letter, stationery::Stationery},
    color::Colors,
    mii_data::MiiData,
    sheet::Sheet,
};
use serde::Serialize;
use serde_bytes::ByteBuf;
use tsify::{Tsify, declare};
use wasm_bindgen::prelude::*;

#[declare]
pub type JsBlocksMap = HashMap<String, Vec<ByteBuf>>;

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    parsing::lzss::decompress_from_slice(bytes).unwrap()
}

#[wasm_bindgen]
pub fn decompress_if_compressed(bytes: &[u8]) -> Vec<u8> {
    parsing::lzss::decompress_from_slice(bytes).unwrap_or_else(|_| bytes.to_vec())
}

fn blocks(v: BlocksHashMap) -> JsBlocksMap {
    v.into_iter()
        .map(|(n, e)| (n, e.into_iter().map(Into::into).collect()))
        .collect()
}

#[derive(Tsify, Serialize)]
pub struct JsStationery {
    name: String,
    background_2d: ByteBuf,
    background_3d: ByteBuf,
    mask: Vec<Vec<u8>>,
    blocks: JsBlocksMap,
}

impl From<Stationery> for JsStationery {
    fn from(value: Stationery) -> Self {
        JsStationery {
            name: value.name,
            background_2d: value.background_2d.into(),
            background_3d: value.background_3d.into(),
            mask: value.mask,
            blocks: blocks(value.blocks),
        }
    }
}

#[derive(Tsify, Serialize)]
pub struct JsMii {
    pub url: String,
    pub name: String,
    pub author_name: String,
}

impl From<MiiData> for JsMii {
    fn from(value: MiiData) -> Self {
        JsMii {
            url: value.get_mii_studio_url(),
            name: value.mii_name,
            author_name: value.creator_name,
        }
    }
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct JsLetter {
    pub thumbnails: Vec<ByteBuf>,
    pub sender_mii: Option<JsMii>,
    pub stationery: Option<JsStationery>,
    pub sheets: Vec<Sheet>,
    pub blocks: JsBlocksMap,
    pub colors: Option<Colors>,
}

#[wasm_bindgen]
pub fn parse_letter(bytes: &[u8]) -> Result<JsLetter, JsError> {
    match Letter::new_from_bpk1_bytes(bytes) {
        Ok(letter) => Ok(JsLetter {
            thumbnails: letter.thumbnails.into_iter().map(Into::into).collect(),
            sender_mii: letter.sender_mii.map(Into::into),
            stationery: letter.stationery.map(Into::into),
            sheets: letter.sheets,
            blocks: blocks(letter.blocks),
            colors: letter.colors,
        }),
        Err(_) => Err(JsError::new("Error reading file")),
    }
}
