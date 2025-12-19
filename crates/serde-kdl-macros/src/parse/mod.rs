//! Parsing modules for KDL components
//!
//! This module contains submodules for parsing different KDL components:
//! - `document`: Document-level parsing
//! - `node`: Node parsing
//! - `value`: Value parsing
//! - `string`: String parsing and validation
//! - `type_annotation`: Type annotation parsing
//! - `number`: Number parsing (Section 3.14)

#[allow(unused)]
pub(crate) mod comments;
pub(crate) mod document;
pub(crate) mod node;
pub(crate) mod type_annotation;
pub(crate) mod value;
#[allow(unused)]
pub(crate) mod whitespace;
