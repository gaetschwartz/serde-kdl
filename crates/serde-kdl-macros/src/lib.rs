//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.
#[cfg(test)]
mod tests;

/// A KDL macro that allows writing KDL syntax directly in Rust code.
///
/// # Examples
///
/// ```rust
/// use serde_kdl_macros::kdl;
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
#[proc_macro]
pub fn kdl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match serde_kdl_macros_internal::kdl_impl(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
