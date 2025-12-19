//! Type annotation parsing
//!
//! This module handles parsing of KDL type annotations according to Section 3.8
//! of the KDL specification.

use crate::ast::KdlIdentifier;
use syn::{parse::ParseStream, Result};

/// Helper function to parse type annotations with whitespace support
/// Supports: (type), ( type ), (multi-word-type), etc.
pub(crate) fn parse_type_annotation(input: ParseStream) -> Result<KdlIdentifier> {
    let content;
    syn::parenthesized!(content in input);

    let content = content.parse::<KdlIdentifier>()?;

    Ok(content)
}
