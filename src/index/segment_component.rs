//! tantivy SegmentComponent mapping
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

use tantivy::index::SegmentComponent;

#[napi(js_name = "SegmentComponent")]
pub enum JsSegmentComponent {
  Postings,
  Positions,
  FastFields,
  FieldNorms,
  Terms,
  Store,
  TempStore,
  Delete,
}

impl From<SegmentComponent> for JsSegmentComponent {
  fn from(value: SegmentComponent) -> Self {
    match value {
      SegmentComponent::Postings => Self::Postings,
      SegmentComponent::Positions => Self::Positions,
      SegmentComponent::FastFields => Self::FastFields,
      SegmentComponent::FieldNorms => Self::FieldNorms,
      SegmentComponent::Terms => Self::Terms,
      SegmentComponent::Store => Self::Store,
      SegmentComponent::TempStore => Self::TempStore,
      SegmentComponent::Delete => Self::Delete,
    }
  }
}

impl From<JsSegmentComponent> for SegmentComponent {
  fn from(value: JsSegmentComponent) -> Self {
    match value {
      JsSegmentComponent::Postings => Self::Postings,
      JsSegmentComponent::Positions => Self::Positions,
      JsSegmentComponent::FastFields => Self::FastFields,
      JsSegmentComponent::FieldNorms => Self::FieldNorms,
      JsSegmentComponent::Terms => Self::Terms,
      JsSegmentComponent::Store => Self::Store,
      JsSegmentComponent::TempStore => Self::TempStore,
      JsSegmentComponent::Delete => Self::Delete,
    }
  }
}
