//! Test that bare identifiers cannot be '.' followed by digits
//!
//! According to Section 3.10.1, patterns like '.5' look like decimal numbers
//! and should not be valid as bare identifiers.

use serde_kdl_macros::kdl;

fn main() {
    // This should fail: looks like a decimal number
    let _ = kdl! {
        .5 "value"
    };
}
