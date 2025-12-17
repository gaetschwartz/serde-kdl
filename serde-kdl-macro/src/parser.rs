//! Main parsing orchestration
//!
//! This module contains the main parsing logic that coordinates between
//! different parsing modules and handles the overall parsing flow.

use crate::ast::KdlDocument;
use crate::codegen::generate_kdl_code;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

pub(crate) fn kdl_impl(input: TokenStream) -> Result<TokenStream2> {
    let document = syn::parse::<KdlDocument>(input)?;
    generate_kdl_code(&document)
}
