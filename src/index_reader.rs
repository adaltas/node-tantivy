//! tantivy IndexReader mapping
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

use napi::Result;
use napi_derive::napi;
use tantivy::IndexReader;

use crate::{napi_err, searcher::JsSearcher};

#[napi(js_name = "IndexReader")]
pub struct JsIndexReader {
  pub(crate) _inner: IndexReader,
}

impl From<IndexReader> for JsIndexReader {
  fn from(_inner: IndexReader) -> Self {
    Self { _inner }
  }
}

#[napi]
impl JsIndexReader {
  #[napi]
  pub fn reload(&self) -> Result<()> {
    self._inner.reload().map_err(napi_err)
  }

  #[napi]
  pub fn searcher(&self) -> JsSearcher {
    self._inner.searcher().into()
  }
}
