//! KDL Specification Tests
//!
//! These tests verify compliance with the KDL specification using the kdl! macro.

pub mod section_3_10_identifiers;
pub mod section_3_11_escapes;
pub mod section_3_14_numbers;
pub mod section_3_15_booleans;
pub mod section_3_16_null;
pub mod section_3_7_values;
pub mod section_3_8_types;
pub mod section_3_9_strings;

pub fn doc_to_string(mut doc: kdl::KdlDocument) -> String {
    doc.autoformat();
    doc.to_string()
}
