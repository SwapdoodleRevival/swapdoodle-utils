use std::{collections::HashMap, io::Read};

use libdoodle::{
    blocks::{
        colslt1::Colors,
        common1::{BasicDateTime, CommonInfo as TrueCommonInfo},
        miistd1::{MiiData, MiiDataBytes},
        sheet1::Sheet,
    },
    bpk1::{BPK1Block, BPK1Blocks, BPK1File},
    error::{GenericError, GenericResult},
    files::stationery::Stationery,
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
pub fn compress_lz10(bytes: &[u8]) -> Vec<u8> {
    libdoodle::lzss::compress_lz10_from_slice(bytes).unwrap()
}

#[wasm_bindgen]
pub fn compress_lz11(bytes: &[u8], max_repeat_size: u32) -> Vec<u8> {
    libdoodle::lzss::compress_lz11_from_slice(bytes, max_repeat_size).unwrap()
}

#[wasm_bindgen]
pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    libdoodle::lzss::decompress_from_slice(bytes).unwrap()
}

#[wasm_bindgen]
pub fn decompress_if_compressed(bytes: &[u8]) -> Vec<u8> {
    libdoodle::lzss::decompress_from_slice(bytes).unwrap_or_else(|_| bytes.to_vec())
}

#[derive(Tsify, Debug, Serialize)]
#[tsify(into_wasm_abi)]
pub struct MiiPreview {
    pub url: String,
    pub name: String,
    pub author_name: String,
}

impl From<MiiData> for MiiPreview {
    fn from(value: MiiData) -> Self {
        MiiPreview {
            url: value.get_mii_studio_url(),
            name: value.mii_name,
            author_name: value.creator_name,
        }
    }
}

fn create_frontend_error(service_name: &str, error: &str) -> JsError {
    JsError::new(format!("{} failed: {}", service_name, error).as_str())
}

#[wasm_bindgen]
pub fn parse_bpk1(bytes: &[u8]) -> Result<Vec<BPK1Block>, JsError> {
    BPK1Blocks::new_from_bpk1_bytes(bytes)
        .map_err(|e| create_frontend_error("BPK1 parser", &e.to_string()))
}

#[wasm_bindgen]
pub fn build_bpk1(blocks: Vec<BPK1Block>) -> Result<Vec<u8>, JsError> {
    BPK1Blocks::bytes_from_bpk1_blocks(blocks)
        .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))
}

#[wasm_bindgen]
pub fn parse_colors(block: &BPK1Block) -> Result<Colors, JsError> {
    Colors::try_from(block.data.as_slice())
        .map_err(|e| create_frontend_error("COLSLT1 parser", &e.to_string()))
}

#[wasm_bindgen]
pub fn parse_sheet(block: &BPK1Block) -> Result<Sheet, JsError> {
    Sheet::try_from(block.data.as_slice())
        .map_err(|e| create_frontend_error("SHEET1 parser", &e.to_string()))
}

#[wasm_bindgen]
pub fn parse_stationery(block: &BPK1Block) -> Result<Stationery, JsError> {
    Stationery::try_from(block.data.as_slice())
        .map_err(|e| create_frontend_error("STATIN1 parser", &e.to_string()))
}

fn read_mii_data(block: &BPK1Block) -> GenericResult<MiiData> {
    let mut mii_data: MiiDataBytes = [0; 0x5C];
    let mut slice: &[u8] = &block.data;
    slice.read(&mut mii_data)?;
    Ok(MiiData::try_from(mii_data)?)
}

#[wasm_bindgen]
pub fn parse_mii_data(block: &BPK1Block) -> Result<MiiPreview, JsError> {
    read_mii_data(block)
        .map(|v| v.into())
        .map_err(|e| create_frontend_error("MIISTD1 parser", &e.to_string()))
}

#[derive(Tsify, Debug, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CommonInfo {
    pub note_id: u128,
    pub reply_to_note_id: u128,
    pub sender_pid: u32,
    pub sent: BasicDateTime,
}

#[wasm_bindgen]
pub fn parse_commoninfo(block: &BPK1Block) -> Result<CommonInfo, JsError> {
    let common_info = TrueCommonInfo::try_from(block.data.as_slice())
        .map_err(|e| create_frontend_error("COMMON1 parser", &e.to_string()))?;
    Ok(CommonInfo {
        note_id: common_info.note_id as u128,
        reply_to_note_id: common_info.reply_to_note_id as u128,
        sender_pid: common_info.sender_pid,
        sent: common_info.sent,
    })
}
