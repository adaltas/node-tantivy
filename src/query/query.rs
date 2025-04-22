//! tantivy QueryParser mapping
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
use tantivy::query::Query;

#[napi(js_name = "Query")]
pub struct JsQuery {
  pub(crate) _inner: Box<dyn Query>,
}

impl From<Box<dyn Query>> for JsQuery {
  fn from(_inner: Box<dyn Query>) -> Self {
    JsQuery { _inner }
  }
}
