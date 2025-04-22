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

use crate::napi_err;

use tantivy::schema::Schema;

use crate::schema::{JsField, JsSchemaBuilder};

#[napi(js_name = "Schema")]
#[derive(Clone, Debug)]
pub struct JsSchema {
  _inner: Schema,
}

#[napi(object)]
pub struct FieldResult {
  pub field_id: u32,
  pub path: String,
}

// Implement conversion from Tantivy's
impl From<Schema> for JsSchema {
  fn from(_inner: Schema) -> Self {
    Self { _inner }
  }
}

impl Into<Schema> for JsSchema {
  fn into(self) -> Schema {
    self._inner
  }
}

#[napi]
impl JsSchema {
  //pub fn get_field_entry(&self, field: Field) -> &FieldEntry

  #[napi]
  pub fn builder(&self) -> JsSchemaBuilder {
    JsSchemaBuilder::from(Schema::builder())
  }

  #[napi]
  pub fn get_field_name(&self, field: &JsField) -> String {
    String::from(self._inner.get_field_name(field._inner))
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
  pub fn find_field(&self, full_path: String) -> Option<FieldResult> {
    self
      ._inner
      .find_field(&full_path)
      .map(|(field, path)| FieldResult {
        field_id: field.field_id(),
        path: path.to_string(),
      })
  }
}
