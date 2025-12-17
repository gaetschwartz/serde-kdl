//! Test that bare identifiers cannot be '-' followed by digits
//!
//! According to Section 3.10.1, patterns like '-42' look like negative numbers
//! and should not be valid as bare identifiers.

use serde_kdl_macro::kdl;

fn main() {
    // This should fail: looks like a negative number
    let _ = kdl! {
        -42 "value"
    };
}
