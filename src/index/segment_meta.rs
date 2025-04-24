//! tantivy SegmentMeta mapping
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

use crate::index::{JsSegmentComponent, JsSegmentId};

#[wrap_struct("tantivy::index::SegmentMeta")]
#[derive(Debug, Clone)]
pub struct JsSegmentMeta;

#[napi]
impl JsSegmentMeta {
  #[napi]
  pub fn id(&self) -> JsSegmentId {
    self._inner.id().into()
  }

  #[napi]
  pub fn untrack_temp_docstore(&self) {
    self._inner.untrack_temp_docstore();
  }

  #[napi]
  pub fn num_deleted_docs(&self) -> u32 {
    self._inner.num_deleted_docs()
  }

  #[napi]
  pub fn list_files(&self) -> Vec<String> {
    self
      ._inner
      .list_files()
      .into_iter()
      .map(|path| path.to_string_lossy().to_string())
      .collect()
  }

  #[napi]
  pub fn relative_path(&self, component: JsSegmentComponent) -> String {
    self
      ._inner
      .relative_path(component.into())
      .to_string_lossy()
      .to_string()
  }

  #[napi]
  pub fn max_doc(&self) -> u32 {
    self._inner.max_doc()
  }

  #[napi]
  pub fn num_docs(&self) -> u32 {
    self._inner.num_docs()
  }

  #[napi]
  pub fn delete_opstamp(&self) -> Option<u64> {
    self._inner.delete_opstamp()
  }

  #[napi]
  pub fn has_deletes(&self) -> bool {
    self._inner.has_deletes()
  }

  #[napi]
  pub fn with_max_doc(&self, max_doc: u32) -> JsSegmentMeta {
    self._inner.clone().with_max_doc(max_doc).into()
  }
}
