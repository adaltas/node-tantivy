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

use inner_wrap::wrap_struct;
use napi::Result;
use napi_derive::napi;

use crate::{napi_err, searcher::JsSearcher};

#[wrap_struct("tantivy::IndexReader")]
pub struct JsIndexReader;

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
