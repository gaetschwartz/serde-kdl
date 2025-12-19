//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.

use proc_macro2::TokenStream as TokenStream2;

use crate::{ast::KdlDocument, expand::generate_kdl_code};

// Module declarations
pub mod ast;
pub mod expand;
pub mod parse;
pub mod validation;

pub fn kdl_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
    let document = syn::parse2::<KdlDocument>(input)?;
    // eprintln!("Parsed KDL Document: {:#?}", document);
    generate_kdl_code(&document)
}
