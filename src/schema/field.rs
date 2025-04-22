//! tantivy Field mapping
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

use tantivy::schema::Field;

#[napi(js_name = "Field")]
pub struct JsField {
  pub(crate) _inner: Field,
}

impl From<Field> for JsField {
  fn from(_inner: Field) -> Self {
    Self { _inner }
  }
}

impl Into<Field> for JsField {
  fn into(self) -> Field {
    self._inner
  }
}

#[napi]
impl JsField {
  #[napi(factory)]
  pub fn from_field_id(field_id: u32) -> Self {
    Field::from_field_id(field_id).into()
  }

  #[napi(getter)]
  pub const fn field_id(&self) -> u32 {
    self._inner.field_id()
  }
}
