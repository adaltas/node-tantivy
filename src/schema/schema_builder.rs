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

use napi_derive::napi;
use tantivy::schema::{JsonObjectOptions, NumericOptions, SchemaBuilder, TextOptions};
use tantivy::schema::{FAST, INDEXED, STORED, STRING, TEXT};
use tantivy::DateOptions;

use crate::schema::JsSchema;

#[napi(js_name = "SchemaBuilder")]
pub struct JsSchemaBuilder {
  _inner: SchemaBuilder,
}

// Implement conversion from Tantivy's
impl From<SchemaBuilder> for JsSchemaBuilder {
  fn from(_inner: SchemaBuilder) -> Self {
    Self { _inner }
  }
}

fn get_num_options(options: Vec<String>) -> NumericOptions {
  let mut num_options = NumericOptions::default();
  for option in options {
    match option.as_str() {
      "STORED" => num_options = num_options | STORED,
      "INDEXED" => num_options = num_options | INDEXED,
      "FAST" => num_options = num_options | FAST, // Enable fast field for numeric range queries
      _ => {}
    }
  }
  num_options
}

#[napi]
impl JsSchemaBuilder {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      _inner: SchemaBuilder::new(),
    }
  }

  /// Adds a text field with indexing options
  #[napi]
  pub fn add_text_field(&mut self, name: String, options: Vec<String>) -> String {
    let mut text_options = TextOptions::default();

    for option in options {
      match option.as_str() {
        "STORED" => text_options = text_options | STORED,
        "STRING" => text_options = STRING,
        "TEXT" => text_options = TEXT,
        _ => {}
      }
    }

    self._inner.add_text_field(&name, text_options);
    name
  }
  /// Adds a u64 (unsigned integer) field
  #[napi]
  pub fn add_u64_field(&mut self, name: String, options: Vec<String>) -> String {
    let num_options = get_num_options(options);
    self._inner.add_u64_field(&name, num_options);
    name
  }

  /// Adds an i64 (signed integer) field
  #[napi]
  pub fn add_i64_field(&mut self, name: String, options: Vec<String>) -> String {
    let num_options = get_num_options(options);
    self._inner.add_i64_field(&name, num_options);
    name
  }

  /// Adds an f64 (floating-point) field
  #[napi]
  pub fn add_f64_field(&mut self, name: String, options: Vec<String>) -> String {
    let num_options = get_num_options(options);
    self._inner.add_f64_field(&name, num_options);
    name
  }

  /// Adds a boolean field
  #[napi]
  pub fn add_bool_field(&mut self, name: String, options: Vec<String>) -> String {
    let mut num_options = NumericOptions::default();
    for option in options {
      //does not support INDEXED
      match option.as_str() {
        "STORED" => num_options = num_options | STORED,
        "FAST" => num_options = num_options | FAST,
        _ => {}
      }
    }
    self._inner.add_bool_field(&name, num_options);
    name
  }

  /// Adds a JSON field
  #[napi]
  pub fn add_json_field(&mut self, name: String, options: Vec<String>) -> String {
    let mut json_options = JsonObjectOptions::default();
    for option in options {
      //does not support FAST
      match option.as_str() {
        "STORED" => json_options = json_options | STORED,
        _ => {}
      }
    }
    self._inner.add_json_field(&name, json_options);
    name
  }

  /// Adds a date field (i64 internally, representing timestamp in microseconds)
  #[napi]
  pub fn add_date_field(&mut self, name: String, options: Vec<String>) -> String {
    let mut date_options = DateOptions::default();
    for option in options {
      match option.as_str() {
        "STORED" => date_options = date_options | STORED,
        "INDEXED" => date_options = date_options | INDEXED,
        "FAST" => date_options = date_options | FAST,
        _ => {}
      }
    }
    self._inner.add_date_field(&name, date_options);
    name
  }

  /// Builds the final schema
  #[napi]
  pub fn build(&mut self) -> JsSchema {
    let inner = std::mem::take(&mut self._inner);
    inner.build().into()
  }
}
