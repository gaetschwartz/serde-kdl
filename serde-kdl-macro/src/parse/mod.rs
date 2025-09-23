//! Parsing modules for KDL components
//!
//! This module contains submodules for parsing different KDL components:
//! - `document`: Document-level parsing
//! - `node`: Node parsing
//! - `value`: Value parsing
//! - `string`: String parsing and validation
//! - `type_annotation`: Type annotation parsing

pub(crate) mod document;
pub(crate) mod node;
pub(crate) mod value;
pub(crate) mod string;
pub(crate) mod type_annotation;
