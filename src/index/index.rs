//! tantivy Index mapping
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

//use napi::bindgen_prelude::*;
use napi::Result;
use napi_derive::napi;
use std::sync::Arc;
use tantivy::Index;

use crate::index::JsIndexMeta;
use crate::napi_err;
use crate::JsIndexReader;

#[napi(js_name = "Index")]
pub struct JsIndex {
  pub(crate) _inner: Arc<Index>,
}

impl Clone for JsIndex {
  fn clone(&self) -> Self {
    JsIndex {
      _inner: Arc::clone(&self._inner),
    }
  }
}

// Implement conversion from Tantivy's
impl From<Index> for JsIndex {
  fn from(_inner: Index) -> Self {
    Self {
      _inner: Arc::new(_inner.clone()),
    }
  }
}

impl From<&Index> for JsIndex {
  fn from(_inner: &Index) -> Self {
    Self {
      _inner: Arc::new(_inner.clone()),
    }
  }
}

#[napi]
impl JsIndex {
  #[napi(factory)]
  pub fn open_in_dir(directory_path: String) -> Result<Self> {
    let index = Index::open_in_dir(directory_path).map_err(napi_err)?;
    Ok(index.into())
  }

  #[napi]
  pub fn reader(&self) -> Result<JsIndexReader> {
    let reader = self._inner.reader().map_err(napi_err)?;
    Ok(reader.into())
  }

  #[napi]
  pub fn load_metas(&self) -> Result<JsIndexMeta> {
    self
      ._inner
      .load_metas()
      .map(JsIndexMeta::from)
      .map_err(napi_err)
  }
  /*
   #[napi]
   pub fn fields_metadata(&self) -> Result<Vec<JsFieldMetadata>> {}
  */
}
