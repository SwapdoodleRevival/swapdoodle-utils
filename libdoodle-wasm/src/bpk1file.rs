use std::{default, ffi::CString, ptr, rc::Rc, str::FromStr};

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

    pub fn reorder_block(
        &mut self,
        moved_block: &FrontendBPK1Block,
        mut new_position: usize,
    ) -> Result<(), JsError> {
        let p = self.blocks.iter().enumerate().find(|(_, k)| {
            moved_block
                .upgrade()
                .map(|u| Rc::ptr_eq(&u, k))
                .unwrap_or(false)
        });
        let (mut index, element) = match p {
            Some(data) => data,
            None => {
                return Err(create_frontend_error(
                    "BPK1File",
                    "Block does not belong to this File",
                ));
            }
        };

        if index != new_position {
            if new_position >= self.blocks.len() {
                new_position = self.blocks.len();
            }

            self.blocks.insert(new_position, element.clone());

            if index > new_position {
                index += 1;
            }

            self.blocks.remove(index);
        }

        Ok(())
    }

    pub fn delete_block(&mut self, to_delete: &FrontendBPK1Block) -> Result<(), JsError> {
        let found = self.blocks.iter().enumerate().find(|(_, el)| {
            (to_delete.upgrade())
                .map(|rc| ptr::eq(el.as_ref(), rc.as_ref()))
                .unwrap_or(false)
        });
        match found {
            Some((index, _)) => {
                self.blocks.remove(index);
                Ok(())
            }
            None => Err(create_frontend_error(
                "BPK1File",
                "Block does not belong to this File",
            )),
        }
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
        Ok(BPK1Blocks::bytes_from_bpk1_blocks(&self.blocks)
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
