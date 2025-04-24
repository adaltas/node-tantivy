//! tantivy IndexSettings mapping
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

use inner_wrap::wrap_struct;
use napi::bindgen_prelude::BigInt;

use crate::store::JsCompressor;

#[wrap_struct("tantivy::index::IndexSettings")]
#[derive(Debug, Clone)]
pub struct JsIndexSettings;

#[napi]
impl JsIndexSettings {
  #[napi(getter)]
  pub fn docstore_compression(&self) -> JsCompressor {
    self._inner.docstore_compression.into()
  }

  #[napi(setter)]
  pub fn set_docstore_compression(&mut self, val: JsCompressor) {
    self._inner.docstore_compression = val.into();
  }

  #[napi(getter)]
  pub fn docstore_blocksize(&self) -> u64 {
    self._inner.docstore_blocksize as u64
  }

  #[napi(setter)]
  pub fn set_docstore_blocksize(&mut self, val: BigInt) {
    let (_, uval, _) = val.get_u64();
    self._inner.docstore_blocksize = uval as usize;
  }
}
