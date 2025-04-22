//! tantivy IndexMeta mapping
//!
//! # License
//!
//! Licensed under the MIT License (see LICENSE file for details).
//!
//! SPDX-License-Identifier: MIT
//!
//! # Author
//!
//! Pierre Sauvage <pierre@adaltas.com>

use tantivy::IndexMeta;

//use crate::schema::JsSchema;

#[derive(Clone, Debug)]
#[napi(js_name = "IndexMeta")]
pub struct JsIndexMeta {
  //todl for a mapping
  pub(crate) _inner: IndexMeta,
  //pub index_settings: JsIndexSettings,
  //pub segments: Vec<SegmentMeta>,
  //pub schema: JsSchema,
  //pub opstamp: Opstamp,
  //pub payload: Option<String>,
}

// Implement conversion from Tantivy's
impl From<IndexMeta> for JsIndexMeta {
  fn from(_inner: IndexMeta) -> Self {
    Self {
      _inner,
      //schema: JsSchema::from(_inner.schema),
      //payload: _inner.payload,
    }
  }
}
