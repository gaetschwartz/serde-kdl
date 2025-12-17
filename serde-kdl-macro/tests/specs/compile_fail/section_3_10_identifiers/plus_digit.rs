//! Test that bare identifiers cannot be '+' followed by digits
//!
//! According to Section 3.10.1, patterns like '+123' look like numbers
//! and should not be valid as bare identifiers.

use serde_kdl_macro::kdl;

fn main() {
    // This should fail: looks like a positive number
    let _ = kdl! {
        +123 "value"
    };
}
