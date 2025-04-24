//! tantivy Compressor mapping
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

use tantivy::store::Compressor;

#[napi(js_name = "Compressor")]
pub enum JsCompressor {
  None,
  Lz4,
}

impl From<Compressor> for JsCompressor {
  fn from(value: Compressor) -> Self {
    match value {
      Compressor::None => Self::None,
      Compressor::Lz4 => Self::Lz4,
    }
  }
}

impl From<JsCompressor> for Compressor {
  fn from(value: JsCompressor) -> Self {
    match value {
      JsCompressor::None => Self::None,
      JsCompressor::Lz4 => Self::Lz4,
    }
  }
}
