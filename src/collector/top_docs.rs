//! tantivy TopDocs mapping
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

use napi::bindgen_prelude::*;
use napi::Result;
use napi_derive::napi;

use tantivy::collector::TopDocs;

use crate::napi_err;
use crate::query::JsQuery;
use crate::{JsDocAddress, JsSearcher};

use tantivy::DocAddress;

#[napi(js_name = "TopDocFruit")]
pub struct JsTopDocFruit {
  pub score: f64,
  pub doc_address: JsDocAddress,
}

impl From<(f32, DocAddress)> for JsTopDocFruit {
  fn from(tuple: (f32, DocAddress)) -> Self {
    JsTopDocFruit {
      score: tuple.0 as f64,
      doc_address: tuple.1.into(),
    }
  }
}

#[napi(js_name = "TopDocs")]
pub struct JsTopDocs {
  _inner: TopDocs,
  limit: usize,
}

#[napi]
impl JsTopDocs {
  #[napi]
  pub fn with_limit(_limit: i64) -> Self {
    let limit = _limit as usize;
    Self {
      _inner: TopDocs::with_limit(limit),
      limit,
    }
  }

  #[napi]
  pub fn and_offset(&self, offset: i64) -> Self {
    let limit = self.limit;
    Self {
      _inner: TopDocs::with_limit(limit).and_offset(offset as usize),
      limit,
    }
  }

  #[napi]
  pub fn search(
    &self,
    query: External<JsQuery>,
    searcher: External<JsSearcher>,
  ) -> Result<Vec<JsTopDocFruit>> {
    let inner_ret = searcher
      ._inner
      .search(&query._inner, &self._inner)
      .map_err(napi_err)?;
    let ret = inner_ret.into_iter().map(|t| t.into()).collect();
    Ok(ret)
  }
}
