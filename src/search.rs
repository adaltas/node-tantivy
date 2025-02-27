//! Contains pure rust+tantivy functions for search
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

use tantivy::collector::TopDocs;
use tantivy::{DocAddress, Index, ReloadPolicy, Searcher, TantivyError};

// PUBLIC

pub fn search(
  index: &Index,
  query: String,
  field_names: Vec<String>,
  limit: usize,
) -> Result<Vec<(f32, DocAddress)>, TantivyError> {
  let reader = get_reader(&index)?;

  let query_parser = get_query_parser(&index, field_names);

  let collector = TopDocs::with_limit(limit);
  let searcher: Searcher = reader.searcher();

  let parsed_query = query_parser.parse_query(&query)?;
  searcher.search(&parsed_query, &collector)
}

// PRIVATE

fn get_reader(index: &Index) -> Result<tantivy::IndexReader, tantivy::TantivyError> {
  index
    .reader_builder()
    .reload_policy(ReloadPolicy::OnCommitWithDelay)
    .try_into()
}

fn get_query_parser(index: &Index, field_names: Vec<String>) -> tantivy::query::QueryParser {
  let schema = index.load_metas().unwrap().schema;
  let fields = field_names
    .iter()
    .map(|x| schema.get_field(x).unwrap())
    .collect();

  tantivy::query::QueryParser::for_index(&index, fields)
}
