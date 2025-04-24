//! tantivy *Options mapping
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

use inner_wrap::*;
use tantivy::schema::{IndexRecordOption, TextFieldIndexing};

use crate::schema::JsDateTimePrecision;

// Re-export Tantivy flags into a shared JS namespace
// Flags are private, must be redefined with same value
// WARNING: Inconsistency risk here ! Update with new tantivy releases
#[napi(namespace = "Options")]
pub const INDEXED: u8 = 0b0000_0001;
#[napi(namespace = "Options")]
pub const STORED: u8 = 0b0000_0010;
#[napi(namespace = "Options")]
pub const FAST: u8 = 0b0000_0100;
#[napi(namespace = "Options")]
pub const COERCE: u8 = 0b0000_1000;
#[napi(namespace = "Options")]
pub const STRING: u8 = 0b0001_0000;
#[napi(namespace = "Options")]
pub const TEXT: u8 = 0b0010_0000;

#[wrap_struct("tantivy::schema::NumericOptions")]
#[derive(Clone)]
pub struct JsNumericOptions;

impl From<u8> for JsNumericOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsNumericOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = NumericOptions::default();
    if flags & INDEXED != 0 {
      _inner = _inner.set_indexed();
    }
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    if flags & FAST != 0 {
      _inner = _inner.set_fast();
    }
    if flags & COERCE != 0 {
      _inner = _inner.set_coerce();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_indexed(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn fieldnorms(&self) -> bool {
    self._inner.fieldnorms()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn should_coerce(&self) -> bool {
    self._inner.should_coerce()
  }

  #[napi]
  pub fn set_coerce(&self) -> Self {
    self._inner.clone().set_coerce().into()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }

  #[napi]
  pub fn set_indexed(&self) -> Self {
    self._inner.clone().set_indexed().into()
  }

  #[napi]
  pub fn set_fieldnorm(&self) -> Self {
    self._inner.clone().set_fieldnorm().into()
  }

  #[napi]
  pub fn set_fast(&self) -> Self {
    self._inner.clone().set_fast().into()
  }
}

// BytesOptions
#[wrap_struct("tantivy::schema::BytesOptions")]
#[derive(Clone)]
pub struct JsBytesOptions;

impl From<u8> for JsBytesOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsBytesOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = BytesOptions::default();
    if flags & INDEXED != 0 {
      _inner = _inner.set_indexed();
    }
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    Self { _inner }
  }
  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_indexed(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn fieldnorms(&self) -> bool {
    self._inner.fieldnorms()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }

  #[napi]
  pub fn set_indexed(&self) -> Self {
    self._inner.clone().set_indexed().into()
  }

  #[napi]
  pub fn set_fieldnorms(&self) -> Self {
    self._inner.clone().set_fieldnorms().into()
  }

  #[napi]
  pub fn set_fast(&self) -> Self {
    self._inner.clone().set_fast().into()
  }
}

// DateOptions
#[wrap_struct("tantivy::schema::DateOptions")]
#[derive(Clone)]
pub struct JsDateOptions;

impl From<u8> for JsDateOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsDateOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = DateOptions::default();
    if flags & INDEXED != 0 {
      _inner = _inner.set_indexed();
    }
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    if flags & FAST != 0 {
      _inner = _inner.set_fast();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_indexed(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn fieldnorms(&self) -> bool {
    self._inner.fieldnorms()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn get_precision(&self) -> JsDateTimePrecision {
    self._inner.get_precision().into()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }

  #[napi]
  pub fn set_indexed(&self) -> Self {
    self._inner.clone().set_indexed().into()
  }

  #[napi]
  pub fn set_fieldnorm(&self) -> Self {
    self._inner.clone().set_fieldnorm().into()
  }

  #[napi]
  pub fn set_fast(&self) -> Self {
    self._inner.clone().set_fast().into()
  }

  #[napi]
  pub fn set_precision(&self, precision: JsDateTimePrecision) -> Self {
    self._inner.clone().set_precision(precision.into()).into()
  }
}

// TextOptions
#[wrap_struct("tantivy::schema::TextOptions")]
#[derive(Clone)]
pub struct JsTextOptions;

impl From<u8> for JsTextOptions {
  fn from(u: u8) -> Self {
    JsTextOptions::from_bits(u)
  }
}

#[napi]
impl JsTextOptions {
  #[napi]
  pub fn from_bits(u: u8) -> Self {
    let mut _inner = TextOptions::default();
    if u & STRING != 0 {
      let mut indexing = TextFieldIndexing::default();
      indexing = indexing.set_tokenizer("raw");
      indexing = indexing.set_index_option(IndexRecordOption::Basic);
      _inner = _inner.set_indexing_options(indexing);
    }
    if u & TEXT != 0 {
      let mut indexing = TextFieldIndexing::default();
      indexing = indexing.set_tokenizer("default");
      indexing = indexing.set_index_option(IndexRecordOption::WithFreqsAndPositions);
      _inner = _inner.set_indexing_options(indexing);
    }
    if u & STORED != 0 {
      _inner = _inner.set_stored();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_fast()
  }

  #[napi]
  pub fn should_coerce(&self) -> bool {
    self._inner.should_coerce()
  }

  #[napi]
  pub fn set_coerce(&self) -> Self {
    self._inner.clone().set_coerce().into()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }
}

// FacetOptions
#[wrap_struct("tantivy::schema::FacetOptions")]
#[derive(Clone)]
pub struct JsFacetOptions;

impl From<u8> for JsFacetOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsFacetOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = FacetOptions::default();
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }
}

// IpAddrOptions
#[wrap_struct("tantivy::schema::IpAddrOptions")]
#[derive(Clone)]
pub struct JsIpAddrOptions;

impl From<u8> for JsIpAddrOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsIpAddrOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = IpAddrOptions::default();
    if flags & INDEXED != 0 {
      _inner = _inner.set_indexed();
    }
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    if flags & FAST != 0 {
      _inner = _inner.set_fast();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_indexed(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn fieldnorms(&self) -> bool {
    self._inner.fieldnorms()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }

  #[napi]
  pub fn set_indexed(&self) -> Self {
    self._inner.clone().set_indexed().into()
  }

  #[napi]
  pub fn set_fieldnorms(&self) -> Self {
    self._inner.clone().set_fieldnorms().into()
  }

  #[napi]
  pub fn set_fast(&self) -> Self {
    self._inner.clone().set_fast().into()
  }
}

// JsonObjectOptions
#[wrap_struct("tantivy::schema::JsonObjectOptions")]
#[derive(Clone)]
pub struct JsJsonObjectOptions;

impl From<u8> for JsJsonObjectOptions {
  fn from(u: u8) -> Self {
    Self::from_bits(u)
  }
}

#[napi]
impl JsJsonObjectOptions {
  #[napi]
  pub fn from_bits(flags: u8) -> Self {
    let mut _inner = JsonObjectOptions::default();
    if flags & STORED != 0 {
      _inner = _inner.set_stored();
    }
    Self { _inner }
  }

  #[napi]
  pub fn is_stored(&self) -> bool {
    self._inner.is_stored()
  }

  #[napi]
  pub fn is_indexed(&self) -> bool {
    self._inner.is_indexed()
  }

  #[napi]
  pub fn is_fast(&self) -> bool {
    self._inner.is_fast()
  }

  #[napi]
  pub fn set_stored(&self) -> Self {
    self._inner.clone().set_stored().into()
  }

  #[napi]
  pub fn set_fast(&self, tokenizer_name: Option<String>) -> Self {
    self
      ._inner
      .clone()
      .set_fast(tokenizer_name.as_ref().map(|v| v.as_str()))
      .into()
  }
}
