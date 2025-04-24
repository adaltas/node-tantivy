//! tantivy FieldEntry mapping
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

//use crate::schema::JsFieldType;

use napi::bindgen_prelude::*;

use inner_wrap::wrap_struct;

use super::options::*;

#[wrap_struct("tantivy::schema::FieldEntry")]
pub struct JsFieldEntry;

#[napi]
impl JsFieldEntry {
  /*
  #[napi(factory)]
  pub fn new(field_name: String, field_type: External<JsFieldType>) -> JsFieldEntry {
    let f = (*field_type).clone().into();
    FieldEntry::new(field_name, f).into()
  }
  */

  #[napi(factory)]
  pub fn new_text(field_name: String, options: External<JsTextOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_text(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_u64(field_name: String, options: External<JsNumericOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_u64(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_i64(field_name: String, options: External<JsNumericOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_i64(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_f64(field_name: String, options: External<JsNumericOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_f64(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_bool(field_name: String, options: External<JsNumericOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_bool(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_date(field_name: String, options: External<JsDateOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_date(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_ip_addr(field_name: String, options: External<JsIpAddrOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_ip_addr(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_facet(field_name: String, options: External<JsFacetOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_facet(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_bytes(field_name: String, options: External<JsBytesOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_bytes(field_name, opts.into()).into()
  }

  #[napi(factory)]
  pub fn new_json(field_name: String, options: External<JsJsonObjectOptions>) -> JsFieldEntry {
    let opts = options.as_ref().clone();
    FieldEntry::new_json(field_name, opts.into()).into()
  }
}
