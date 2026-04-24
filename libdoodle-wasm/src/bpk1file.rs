use std::{
    cell::{Ref, RefCell},
    ffi::CString,
    ops::Deref,
    rc::Rc,
    str::FromStr,
};

use libdoodle::bpk1::{BPK1Block, BPK1Blocks, BPK1File};
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::{bpk1block::FrontendBPK1Block, create_frontend_error};

#[derive(Default)]
#[wasm_bindgen(js_name = BackendBPK1File)]
pub struct FrontendBPK1File {
    blocks: Vec<Rc<RefCell<BPK1Block>>>,
}

#[wasm_bindgen(js_class = BackendBPK1File)]
impl FrontendBPK1File {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_bpk1_bytes(bytes: Vec<u8>) -> Result<Self, JsError> {
        Ok(Self {
            blocks: BPK1Blocks::new_from_bpk1_bytes(&bytes)
                .map_err(|e| create_frontend_error("BPK1 parser", &e.to_string()))?
                .into_iter()
                .map(|k| RefCell::new(k).into())
                .collect(),
        })
    }

    pub fn get_blocks(&self) -> Vec<FrontendBPK1Block> {
        self.blocks
            .iter()
            .map(|item| FrontendBPK1Block {
                block: Rc::downgrade(item),
            })
            .collect()
    }

    pub fn reorder_block(
        &mut self,
        block_position: usize,
        new_position: usize,
    ) -> Result<(), JsError> {
        let max = usize::max(block_position, new_position);

        if max >= self.blocks.len() {
            return Err(create_frontend_error("BPK1File", "Index out of range"));
        }

        if block_position != new_position {
            let min = usize::min(block_position, new_position);
            let inner_slice = &mut self.blocks[min..=max];
            if block_position < new_position {
                inner_slice.rotate_left(1);
            } else {
                inner_slice.rotate_right(1);
            }
        }

        Ok(())
    }

    pub fn delete_block(&mut self, to_delete: &FrontendBPK1Block) -> Result<(), JsError> {
        let block = to_delete.upgrade()?;
        let found = self
            .blocks
            .iter()
            .enumerate()
            .find(|(_, el)| Rc::ptr_eq(&block, el));
        let (index, _) = found.ok_or_else(|| {
            create_frontend_error("BPK1File", "Block does not belong to this File")
        })?;
        self.blocks.remove(index);
        Ok(())
    }

    pub fn push_bpk1_block(&mut self, name: String, bytes: Vec<u8>) -> Result<(), JsError> {
        self.blocks.push(
            RefCell::new(BPK1Block {
                name: CString::from_str(&name)
                    .map_err(|e| create_frontend_error("BPK1 parser", &e.to_string()))?,
                data: bytes,
            })
            .into(),
        );
        Ok(())
    }

    pub fn build_uncompressed_bpk1_archive(&mut self) -> Result<Vec<u8>, JsError> {
        let refs = &self
            .blocks
            .iter()
            .map(|v| v.deref().borrow())
            .collect::<Vec<Ref<BPK1Block>>>();
        BPK1Blocks::bytes_from_bpk1_blocks(
            &refs.iter().map(|v| v.deref()).collect::<Vec<&BPK1Block>>(),
        )
        .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))
    }

    pub fn build_lz11_bpk1_archive(&mut self, max_repeat_size: u32) -> Result<Vec<u8>, JsError> {
        libdoodle::lzss::compress_lz11_from_slice(
            &self.build_uncompressed_bpk1_archive()?,
            max_repeat_size,
        )
        .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))
    }

    pub fn build_lz10_bpk1_archive(&mut self) -> Result<Vec<u8>, JsError> {
        libdoodle::lzss::compress_lz10_from_slice(&self.build_uncompressed_bpk1_archive()?)
            .map_err(|e| create_frontend_error("BPK1 serializer", &e.to_string()))
    }
}
