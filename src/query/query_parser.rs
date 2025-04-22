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

use napi::bindgen_prelude::*;

use napi_derive::napi;
use tantivy::query::QueryParser;
use tantivy::schema::Field;

use crate::index::JsIndex;
use crate::napi_err;
use crate::query::JsQuery;

#[napi(js_name = "QueryParser")]
pub struct JsQueryParser {
  _inner: QueryParser,
}

#[napi]
impl JsQueryParser {
  /// Static method to create a new QueryParser
  #[napi(factory)]
  pub fn for_index(index: &JsIndex, fields: Vec<External<Field>>) -> Self {
    /*
    f is &External<Field>
    *f is External<Field> (you dereference the reference)
    **f is Field (you dereference the external)
    */
    let _fields: Vec<Field> = fields.iter().map(|f| **f).collect();
    let _inner = QueryParser::for_index(index._inner.as_ref(), _fields);
    Self { _inner }
  }

  /// Parses a query string
  #[napi]
  pub fn parse_query(&self, query: String) -> napi::Result<JsQuery> {
    self
      ._inner
      .parse_query(&query)
      .map(|query| query.into())
      .map_err(napi_err)
  }
}
