//! tantivy searcher mapping
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

use napi_derive::napi;
use tantivy::Searcher;

use crate::{index::JsIndex, schema::JsSchema};

#[napi(js_name = "Searcher")]
pub struct JsSearcher {
  pub(crate) _inner: Searcher,
}

// Implement conversion from Tantivy's
impl From<Searcher> for JsSearcher {
  fn from(_inner: Searcher) -> Self {
    Self { _inner }
  }
}

#[napi]
impl JsSearcher {
  #[napi]
  pub fn index(&self) -> JsIndex {
    self._inner.index().into()
  }

  pub fn schema(&self) -> JsSchema {
    self._inner.schema().clone().into()
  }
}
