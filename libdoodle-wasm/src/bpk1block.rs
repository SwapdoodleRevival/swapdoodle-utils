use std::{
    io::Read,
    rc::{Rc, Weak},
};

use libdoodle::{
    blocks::{
        colslt1::Colors,
        common1::BasicDateTime,
        miistd1::{MiiData, MiiDataBytes},
        sheet1::Sheet,
    },
    bpk1::BPK1Block,
    files::stationery::Stationery,
};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::create_frontend_error;

#[wasm_bindgen(js_name = BackendBPK1Block)]
pub struct FrontendBPK1Block {
    pub(crate) block: Weak<BPK1Block>,
}

#[derive(Tsify, Debug, Serialize)]
#[tsify(into_wasm_abi)]
// Using u128 to enforce the use of BigInt
// This is a wasm-bindgen bug :(
pub struct CommonInfo {
    pub note_id: u128,
    pub reply_to_note_id: u128,
    pub sender_pid: u32,
    pub sent: BasicDateTime,
}

#[derive(Tsify, Debug, Serialize)]
#[tsify(into_wasm_abi)]
// A lightweight Mii Preview that only has the Studio URL and names
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

impl FrontendBPK1Block {
    pub fn upgrade(&self) -> Result<Rc<BPK1Block>, JsError> {
        match self.block.upgrade() {
            Some(block) => Ok(block),
            None => Err(create_frontend_error("BPK1Block", "Reference has expired")),
        }
    }
}

#[wasm_bindgen(js_class = BackendBPK1Block)]
impl FrontendBPK1Block {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Result<String, JsError> {
        Ok(self.upgrade()?.name.to_str()?.to_string())
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<Vec<u8>, JsError> {
        Ok(self.upgrade()?.data.clone())
    }

    pub fn is_equal(&self, rhs: &FrontendBPK1Block) -> bool {
        self.block.ptr_eq(&rhs.block)
    }

    pub fn parse_colors(&self) -> Result<Colors, JsError> {
        Colors::try_from(self.upgrade()?.data.as_slice())
            .map_err(|e| create_frontend_error("COLSLT1 parser", &e.to_string()))
    }

    pub fn parse_sheet(&self) -> Result<Sheet, JsError> {
        Sheet::try_from(self.upgrade()?.data.as_slice())
            .map_err(|e| create_frontend_error("SHEET1 parser", &e.to_string()))
    }

    pub fn parse_stationery(&self) -> Result<Stationery, JsError> {
        Stationery::try_from(self.upgrade()?.data.as_slice())
            .map_err(|e| create_frontend_error("STATIN1 parser", &e.to_string()))
    }

    pub fn parse_mii_data(&self) -> Result<MiiPreview, JsError> {
        let mut mii_data: MiiDataBytes = [0; 0x5C];
        let mut slice: &[u8] = &self.upgrade()?.data;
        slice.read(&mut mii_data)?;
        MiiData::try_from(mii_data)
            .map(|v| v.into())
            .map_err(|e| create_frontend_error("MIISTD1 parser", &e.to_string()))
    }

    pub fn parse_commoninfo(&self) -> Result<CommonInfo, JsError> {
        let common_info =
            libdoodle::blocks::common1::CommonInfo::try_from(self.upgrade()?.data.as_slice())
                .map_err(|e| create_frontend_error("COMMON1 parser", &e.to_string()))?;
        Ok(CommonInfo {
            note_id: common_info.note_id as u128,
            reply_to_note_id: common_info.reply_to_note_id as u128,
            sender_pid: common_info.sender_pid,
            sent: common_info.sent,
        })
    }
}
