//! tantivy FieldType mapping
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

//use crate::schema::options::*;

//use napi::bindgen_prelude::*;

/*
#[napi(js_name = "Type")]
pub enum JsType {
  Str = 115,
  U64 = 117,
  I64 = 105,
  F64 = 102,
  Bool = 111,
  Date = 100,
  Facet = 104,
  Bytes = 98,
  Json = 106,
  IpAddr = 112,
}

#[derive(Clone)]
#[napi(js_name = "FieldType")]
pub struct JsFieldType {
  pub typef: JsType,
  text_options: Option<JsTextOptions>,
  numeric_options: Option<JsNumericOptions>,
  date_options: Option<JsDateOptions>,
  facet_options: Option<JsFacetOptions>,
  bytes_options: Option<JsBytesOptions>,
  json_object_options: Option<JsJsonObjectOptions>,
  ip_addr_options: Option<JsIpAddrOptions>,
}

impl From<FieldType> for JsFieldType {
  fn from(value: FieldType) -> Self {
    match value {
      FieldType::Str(v) => Self { type: JsType.Str},
      FieldType::U64(v) => Self::U64(v.into()),
      FieldType::I64(v) => Self::I64(v.into()),
      FieldType::F64(v) => Self::F64(v.into()),
      FieldType::Bool(v) => Self::Bool(v.into()),
      FieldType::Date(v) => Self::Date(v.into()),
      FieldType::Facet(v) => Self::Facet(v.into()),
      FieldType::Bytes(v) => Self::Bytes(v.into()),
      FieldType::JsonObject(v) => Self::JsonObject(v.into()),
      FieldType::IpAddr(v) => Self::IpAddr(v.into()),
    }
  }
}

impl From<JsFieldType> for FieldType {
  fn from(value: JsFieldType) -> Self {
    match value {
      JsFieldType::Str(v) => Self::Str(v.into()),
      JsFieldType::U64(v) => Self::U64(v.into()),
      JsFieldType::I64(v) => Self::I64(v.into()),
      JsFieldType::F64(v) => Self::F64(v.into()),
      JsFieldType::Bool(v) => Self::Bool(v.into()),
      JsFieldType::Date(v) => Self::Date(v.into()),
      JsFieldType::Facet(v) => Self::Facet(v.into()),
      JsFieldType::Bytes(v) => Self::Bytes(v.into()),
      JsFieldType::JsonObject(v) => Self::JsonObject(v.into()),
      JsFieldType::IpAddr(v) => Self::IpAddr(v.into()),
    }
  }
}

*/
