//! tantivy SegmentId mapping
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

use crate::napi_err;

#[wrap_struct("tantivy::index::SegmentId")]
#[derive(Debug, Clone)]
pub struct JsSegmentId;

#[napi]
impl JsSegmentId {
  #[napi]
  pub fn short_uuid_string(&self) -> String {
    self._inner.short_uuid_string()
  }

  #[napi]
  pub fn uuid_string(&self) -> String {
    self._inner.uuid_string()
  }

  #[napi(factory)]
  pub fn from_uuid_string(uuid_string: String) -> napi::Result<JsSegmentId> {
    SegmentId::from_uuid_string(uuid_string.as_str())
      .map(JsSegmentId::from)
      .map_err(napi_err)
  }
}
