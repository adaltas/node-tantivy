//! Contains all functions callable in JS
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

#[macro_use]
extern crate napi_derive;

mod collector;
mod index;
mod index_reader;
mod old;
mod oldsearch;
mod query;
mod schema;
mod searcher;
mod structs;

pub use index_reader::JsIndexReader;
pub use old::*;
pub use searcher::JsSearcher;
pub use structs::*;

pub fn napi_err<E: std::fmt::Display>(e: E) -> napi::Error {
  napi::Error::from_reason(e.to_string())
}
