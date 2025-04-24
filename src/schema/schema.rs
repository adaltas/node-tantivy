//! tantivy Schema mapping
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
use tantivy::schema::Field;

use inner_wrap::wrap_struct;

use crate::napi_err;

use crate::schema::{JsField, JsSchemaBuilder};

#[wrap_struct("tantivy::schema::Schema")]
pub struct JsSchema;

#[napi(object, js_name = "FieldResult")]
pub struct JsFieldResult {
  pub field_id: u32,
  pub path: String,
}

impl From<(Field, &str)> for JsFieldResult {
  fn from(tuple: (Field, &str)) -> Self {
    Self {
      field_id: tuple.0.field_id(),
      path: tuple.1.to_string(),
    }
  }
}

#[napi]
impl JsSchema {
  //pub fn get_field_entry(&self, field: Field) -> &FieldEntry

  #[napi]
  pub fn builder(&self) -> JsSchemaBuilder {
    Schema::builder().into()
  }

  #[napi]
  pub fn get_field_name(&self, field: &JsField) -> String {
    self._inner.get_field_name(field._inner).into()
  }

  #[napi]
  pub fn num_fields(&self) -> usize {
    self._inner.num_fields()
  }

  #[napi]
  pub fn get_field(&self, field_name: String) -> Result<JsField> {
    self
      ._inner
      .get_field(field_name.as_str())
      .map(JsField::from)
      .map_err(napi_err)
  }

  #[napi]
  pub fn find_field(&self, full_path: String) -> Option<JsFieldResult> {
    self._inner.find_field(&full_path).map(|t| t.into())
  }
}
