//! js macro mapping
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

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

use syn::{parse_macro_input, punctuated::Punctuated, token::Comma};

#[proc_macro_attribute]
pub fn wrap_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
  // Parse l’attribut #[wrap_js("crate::types::FieldEntry")]
  let args = parse_macro_input!(attr with Punctuated::<Expr, Comma>::parse_terminated);
  let mut input = parse_macro_input!(item as ItemStruct);

  let type_path_str = match args.first() {
    Some(Expr::Lit(ExprLit {
      lit: Lit::Str(s), ..
    })) => s.value(),
    _ => {
      return syn::Error::new_spanned(
        &args.first(),
        "Expected string literal like #[wrap_js(\"crate::some::Type\")]",
      )
      .to_compile_error()
      .into();
    }
  };

  let import_path: Path = match syn::parse_str(&type_path_str) {
    Ok(p) => p,
    Err(e) => return e.to_compile_error().into(),
  };

  let imported_ident = match import_path.segments.last() {
    Some(seg) => &seg.ident,
    None => {
      return syn::Error::new_spanned(&args.first(), "Invalid path: no type name found")
        .to_compile_error()
        .into();
    }
  };

  // Inject #[napi(js_name = "...")]
  let js_name_lit = imported_ident.to_string();
  let napi_attr: syn::Attribute = syn::parse_quote! {
      #[napi(js_name = #js_name_lit)]
  };
  input.attrs.push(napi_attr);

  // Inject field: pub(crate) _inner: FieldEntry
  let field: syn::Field = syn::parse_quote! {
      pub(crate) _inner: #import_path
  };
  input.fields = syn::Fields::Named(syn::FieldsNamed {
    brace_token: Default::default(),
    named: std::iter::once(field).collect(),
  });

  let wrapper_ident = &input.ident;

  let expanded = quote! {
      use #import_path;

      #input

      impl From<#import_path> for #wrapper_ident {
          fn from(_inner: #import_path) -> Self {
              Self { _inner }
          }
      }

      impl From<#wrapper_ident> for #import_path {
          fn from(s: #wrapper_ident) -> Self {
              s._inner
          }
      }
  };

  expanded.into()
}
