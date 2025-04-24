//! tantivy SchemaBuilder mapping
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
use napi_derive::napi;

use napi::bindgen_prelude::*;
use tantivy::schema::{
  DateOptions, FacetOptions, IpAddrOptions, JsonObjectOptions, NumericOptions, TextOptions,
};

use crate::schema::*;

#[wrap_struct("tantivy::schema::SchemaBuilder")]
pub struct JsSchemaBuilder;

#[napi]
impl JsSchemaBuilder {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      _inner: SchemaBuilder::new(),
    }
  }

  /// Adds a u64 (unsigned integer) field
  #[napi]
  pub fn add_u64_field(&mut self, name: String, options: External<JsNumericOptions>) -> JsField {
    let opts: NumericOptions = options.as_ref().clone().into();
    self._inner.add_u64_field(&name, opts).into()
  }

  /// Adds an i64 (signed integer) field
  #[napi]
  pub fn add_i64_field(&mut self, name: String, options: External<JsNumericOptions>) -> JsField {
    let opts: NumericOptions = options.as_ref().clone().into();
    self._inner.add_i64_field(&name, opts).into()
  }

  /// Adds an f64 (floating-point) field
  #[napi]
  pub fn add_f64_field(&mut self, name: String, options: External<JsNumericOptions>) -> JsField {
    let opts: NumericOptions = options.as_ref().clone().into();
    self._inner.add_f64_field(&name, opts).into()
  }

  /// Adds a boolean field
  #[napi]
  pub fn add_bool_field(&mut self, name: String, options: External<JsNumericOptions>) -> JsField {
    let opts: NumericOptions = options.as_ref().clone().into();
    self._inner.add_bool_field(&name, opts).into()
  }

  /// Adds a date field (i64 internally, representing timestamp in microseconds)
  #[napi]
  pub fn add_date_field(&mut self, name: String, options: External<JsDateOptions>) -> JsField {
    let opts: DateOptions = options.as_ref().clone().into();
    self._inner.add_date_field(&name, opts).into()
  }

  #[napi]
  pub fn add_ip_addr_field(&mut self, name: String, options: External<JsIpAddrOptions>) -> JsField {
    let opts: IpAddrOptions = options.as_ref().clone().into();
    self._inner.add_ip_addr_field(&name, opts).into()
  }

  /// Adds a text field with indexing options
  #[napi]
  pub fn add_text_field(&mut self, name: String, options: External<JsTextOptions>) -> JsField {
    let opts: TextOptions = options.as_ref().clone().into();
    self._inner.add_text_field(&name, opts).into()
  }

  /// Adds a JSON field
  #[napi]
  pub fn add_facet_field(&mut self, name: String, options: External<JsFacetOptions>) -> JsField {
    let opts: FacetOptions = options.as_ref().clone().into();
    self._inner.add_facet_field(&name, opts).into()
  }

  /// Adds a JSON field
  #[napi]
  pub fn add_json_field(
    &mut self,
    name: String,
    options: External<JsJsonObjectOptions>,
  ) -> JsField {
    let opts: JsonObjectOptions = options.as_ref().clone().into();
    self._inner.add_json_field(&name, opts).into()
  }

  /// Builds the final schema
  #[napi]
  pub fn build(&mut self) -> JsSchema {
    let inner = std::mem::take(&mut self._inner);
    inner.build().into()
  }
}
