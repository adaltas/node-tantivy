//! schema mod
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

//do not implement for now
//pub mod schema_builder;

mod field;
mod schema;
mod schema_builder;

pub use field::JsField;
pub use schema::JsSchema;
pub use schema_builder::JsSchemaBuilder;
