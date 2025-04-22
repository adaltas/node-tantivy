//! Contains old initial API functions
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

//NAPI-RS
use napi::bindgen_prelude::*;

use crate::{oldsearch, JsDocAddress};

use crate::{collector::JsTopDocFruit, JsSearchOptions};

//STD
use std::path::Path;
//TANTIVY
use tantivy::{Index, TantivyDocument};

#[napi]
pub fn open_in_dir(path_str: String) -> Result<External<Index>> {
  //let path_str = "./index";
  let path = Path::new(&path_str);
  let index = Index::open_in_dir(&path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
  Ok(External::new(index))
}

#[napi]
pub fn search(
  index: External<Index>,
  query: String,
  search_options: Option<JsSearchOptions>,
) -> Result<Vec<JsTopDocFruit>> {
  let options = search_options.unwrap_or(JsSearchOptions {
    fields: None, // Don't compute default_fields here
    limit: None,
  });

  let limit = options.limit.unwrap_or(10);
  //Check limit is positive
  if limit <= 0 {
    return Err(napi::Error::new(
      Status::InvalidArg,
      "Limit must be strictly positive",
    ));
  }
  //get fields or get default (all indexed) of index
  let field_names = options
    .fields
    .unwrap_or_else(|| oldsearch::default_fields(&index).unwrap());

  let results = oldsearch::search(&index, query, field_names, limit as usize);

  match results {
    Ok(mut docs) => {
      //sort in DESCENDING order (compare b to a)
      docs.sort_by(|(score_a, _), (score_b, _)| score_b.partial_cmp(score_a).unwrap());
      Ok(docs.into_iter().map(|t| JsTopDocFruit::from(t)).collect())
    }
    Err(e) => Err(napi::Error::from_reason(e.to_string())),
  }
}

#[napi]
pub fn get_document_by_address(
  index: External<Index>,
  doc_address: JsDocAddress,
) -> napi::Result<String> {
  let reader = index
    .reader()
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

  let searcher = reader.searcher();
  let doc_address: tantivy::DocAddress = doc_address.into();

  let segment_reader = searcher.segment_reader(doc_address.segment_ord);
  let store_reader = segment_reader.get_store_reader(100)?;

  let stored_doc: TantivyDocument = store_reader
    .get(doc_address.doc_id)
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

  // Sérialisation du document en JSON
  serde_json::to_string(&stored_doc).map_err(|e| napi::Error::from_reason(e.to_string()))
}
