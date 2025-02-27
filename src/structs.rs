//! Contains all NAPI struct / JsObject
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

use tantivy::DocAddress;

#[napi(object)]
pub struct JsDocAddress {
  pub segment_ord: u32,
  pub doc_id: u32,
}
// Implement conversion to Tantivy's `DocAddress`
impl From<JsDocAddress> for DocAddress {
  fn from(js_addr: JsDocAddress) -> Self {
    DocAddress::new(js_addr.segment_ord, js_addr.doc_id)
  }
}

// Implement conversion from Tantivy's `DocAddress` to `JsDocAddress`
impl From<DocAddress> for JsDocAddress {
  fn from(tantivy_addr: DocAddress) -> Self {
    JsDocAddress {
      segment_ord: tantivy_addr.segment_ord,
      doc_id: tantivy_addr.doc_id,
    }
  }
}

//Custom objects

#[napi(object)]
pub struct TopDoc {
  pub score: f64,
  pub doc_address: JsDocAddress,
}
