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
use tantivy::{IndexReaderBuilder, ReloadPolicy};

use crate::{napi_err, JsIndexReader};
use std::sync::Mutex;

#[napi(js_name = "IndexReaderBuilder")]
pub struct JsIndexReaderBuilder {
  _inner: Mutex<IndexReaderBuilder>,
}

impl From<IndexReaderBuilder> for JsIndexReaderBuilder {
  fn from(_inner: IndexReaderBuilder) -> Self {
    Self {
      _inner: Mutex::new(_inner),
    }
  }
}

#[napi]
impl JsIndexReaderBuilder {
  #[napi]
  pub fn try_into(&self) -> Result<JsIndexReader> {
    let guard = self._inner.lock().unwrap().clone();
    let index_reader = guard.try_into().map_err(napi_err)?;
    Ok(index_reader.into())
  }

  #[napi]
  pub fn reload_policy(&self, policy: String) -> Result<Self> {
    let inner = self._inner.lock().unwrap().clone();

    let parsed_policy = match policy.as_str() {
      "OnCommitWithDelay" => ReloadPolicy::OnCommitWithDelay,
      "Manual" => ReloadPolicy::Manual,
      _ => return Err(napi::Error::from_reason("Invalid reload policy")),
    };

    Ok(Self {
      _inner: Mutex::new(inner.reload_policy(parsed_policy)), // `IndexReaderBuilder: Clone`
    })
  }

  #[napi]
  pub fn doc_store_cache_num_blocks(&self, doc_store_cache_num_blocks: u32) -> Result<Self> {
    let inner = self._inner.lock().unwrap().clone();
    Ok(Self {
      _inner: Mutex::new(inner.doc_store_cache_num_blocks(doc_store_cache_num_blocks as usize)),
    })
  }

  //#[napi]
  //pub fn warmers(&self, warmers: Vec<std::rc::Weak<dyn tantivy::Warmer>>) -> Self;

  #[napi]
  pub fn num_warming_threads(&self, num_warming_threads: u32) -> Result<Self> {
    let inner = self._inner.lock().unwrap().clone();
    Ok(Self {
      _inner: Mutex::new(inner.num_warming_threads(num_warming_threads as usize)),
    })
  }
}
