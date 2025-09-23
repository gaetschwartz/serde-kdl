//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.

use proc_macro::TokenStream;

// Module declarations
mod ast;
mod codegen;
mod parse;
mod parser;
mod utils;
mod validation;

#[cfg(test)]
mod specs;

/// A KDL macro that allows writing KDL syntax directly in Rust code.
///
/// # Examples
///
/// ```rust
/// use serde_kdl_macro::kdl;
///
/// // Simple node with value
/// let doc = kdl! { node 42 };
///
/// // Node with properties
/// let doc = kdl! { node key="value" 42 };
///
/// // Nested nodes
/// let doc = kdl! {
///     parent {
///         child "value"
///     }
/// };
/// ```
///
/// # Line Continuation Support
///
/// This macro supports KDL line continuations (Section 3.3 of the KDL specification).
/// Line continuations allow nodes to be spread across multiple lines using a backslash
/// followed by optional whitespace/comments and a newline.
///
/// Note: Due to Rust's lexer limitations, line continuations in the macro syntax
/// are processed through string preprocessing before parsing.
#[proc_macro]
pub fn kdl(input: TokenStream) -> TokenStream {
    match parser::kdl_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
