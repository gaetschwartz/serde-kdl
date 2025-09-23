//! Main parsing orchestration
//!
//! This module contains the main parsing logic that coordinates between
//! different parsing modules and handles the overall parsing flow.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;
use crate::ast::KdlDocument;
use crate::utils::process_line_continuation_string;
use crate::codegen::generate_kdl_code;

pub(crate) fn kdl_impl(input: TokenStream) -> Result<TokenStream2> {
    // Preprocess input to handle line continuations
    let input_str = input.to_string();
    let processed_str = process_line_continuation_string(&input_str)?;

    // Parse the processed string back into TokenStream and then to KdlDocument
    let processed_tokens: TokenStream = processed_str.parse()
        .map_err(|e| syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to parse processed KDL: {}", e),
        ))?;

    let document = syn::parse::<KdlDocument>(processed_tokens)?;

    generate_kdl_code(&document)
}
