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

use inner_wrap::wrap_struct;

use crate::index::{JsIndexSettings, JsSegmentMeta};
use crate::schema::JsSchema;

#[wrap_struct("tantivy::IndexMeta")]
#[derive(Clone, Debug)]
pub struct JsIndexMeta;

#[napi]
impl JsIndexMeta {
  #[napi(getter)]
  pub fn index_settings(&self) -> JsIndexSettings {
    self._inner.index_settings.clone().into()
  }

  #[napi(getter)]
  pub fn segments(&self) -> Vec<JsSegmentMeta> {
    self
      ._inner
      .segments
      .iter()
      .map(|s| s.clone().into())
      .collect()
  }

  #[napi(getter)]
  pub fn schema(&self) -> JsSchema {
    self._inner.schema.clone().into()
  }

  #[napi(getter)]
  pub fn opstamp(&self) -> u64 {
    self._inner.opstamp
  }

  #[napi(getter)]
  pub fn payload(&self) -> Option<String> {
    self._inner.payload.clone()
  }
}
