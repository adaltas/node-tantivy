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

mod date_time_precision;
mod field;
mod field_entry;
//mod field_type;
mod options;
mod schema;
mod schema_builder;

pub use date_time_precision::JsDateTimePrecision;
pub use field::JsField;
pub use options::*;

pub use schema::JsSchema;
pub use schema_builder::JsSchemaBuilder;
