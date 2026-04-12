use std::{default, ffi::CString, rc::Rc, str::FromStr};

use libdoodle::bpk1::{BPK1Block, BPK1Blocks, BPK1File};
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::{bpk1block::FrontendBPK1Block, create_frontend_error};

#[wasm_bindgen(js_name = BPK1File)]
pub struct FrontendBPK1File {
    blocks: Vec<Rc<BPK1Block>>,
}

#[wasm_bindgen(js_class = BPK1File)]
impl FrontendBPK1File {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            blocks: default::Default::default(),
        }
    }

    pub fn from_bpk1_bytes(bytes: Vec<u8>) -> Result<Self, JsError> {
        Ok(Self {
            blocks: BPK1Blocks::new_from_bpk1_bytes(&bytes)
                .map_err(|e| create_frontend_error("BPK1 parser", &e.to_string()))?
                .into_iter()
                .map(|k| k.into())
                .collect(),
        })
    }

    pub fn get_blocks(&self) -> Result<Vec<FrontendBPK1Block>, JsError> {
        let mut result = Vec::<FrontendBPK1Block>::new();
        for item in &self.blocks {
            result.push(FrontendBPK1Block {
                block: Rc::downgrade(item),
            });
        }
        Ok(result)
    }

    pub fn insert_bpk1_block(&mut self, name: String, bytes: Vec<u8>) -> Result<(), JsError> {
        self.blocks.push(
            (BPK1Block {
                name: CString::from_str(&name)
                    .map_err(|e| create_frontend_error("BPK1 parser", &e.to_string()))?,
                data: bytes,
            })
            .into(),
        );
        Ok(())
    }

    pub fn to_uncompressed_bpk1_archive(&mut self) -> Result<Vec<u8>, JsError> {
        Ok(BPK1Blocks::bytes_from_bpk1_blocks(
            &self
                .blocks
                .iter()
                .map(|b| b.as_ref())
                .collect::<Vec<&BPK1Block>>(),
        )
        .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))?)
    }

    pub fn to_lz11_bpk1_archive(&mut self, max_repeat_size: u32) -> Result<Vec<u8>, JsError> {
        Ok(libdoodle::lzss::compress_lz11_from_slice(
            &self.to_uncompressed_bpk1_archive()?,
            max_repeat_size,
        )
        .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))?)
    }

    pub fn to_lz10_bpk1_archive(&mut self) -> Result<Vec<u8>, JsError> {
        Ok(
            libdoodle::lzss::compress_lz10_from_slice(&self.to_uncompressed_bpk1_archive()?)
                .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))?,
        )
    }
}
