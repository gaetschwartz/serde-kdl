//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.

use proc_macro::TokenStream;
use serde_kdl_macros_internal::kdl_impl;

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
pub fn kdl(input: TokenStream) -> TokenStream {
    match kdl_impl(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

// #[cfg(test)]
// mod tests {
//     use bolero::check;
//     use serde_kdl_macros_internal::ast;

//     const _: () = {
//         const fn impls_arbitrary<'a, T: arbitrary::Arbitrary<'a>>() {}
//         impls_arbitrary::<kdl::KdlDocument>();
//     };

//     #[test]
//     fn test_sip_parsing_roundtrip() {
//         check!()
//             .with_arbitrary::<kdl::KdlDocument>()
//             .for_each(|packet| {
//                 let kdl_str = packet.to_string();
//                 let visible_count = kdl_str.chars().filter(|c| c.is_ascii_graphic()).count();
//                 if visible_count == 0 || visible_count > 10_000 {
//                     // Skip too small or too large inputs
//                     return;
//                 }
//                 let tokenstream = match syn::parse_str::<proc_macro2::TokenStream>(&kdl_str) {
//                     Ok(tokenstream) => tokenstream,
//                     Err(e) => {
//                         eprintln!("Failed to parse generated KDL string into TokenStream: {e}\nSource:\n{kdl_str}",);
//                         return;
//                     }
//                 };
//                 let parsed = syn::parse2::<ast::KdlDocument>(tokenstream).unwrap_or_else(|e| {
//                     panic!("Failed to parse generated KDL string: {e}\nSource:\n{kdl_str}",)
//                 });
//                 assert_eq!(&parsed, packet, "Roundtrip mismatch for KDL string:\n{kdl_str}");
//             });
//     }
// }
