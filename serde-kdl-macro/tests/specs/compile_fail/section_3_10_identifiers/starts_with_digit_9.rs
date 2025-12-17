//! Test that bare identifiers cannot start with a digit (9)
//!
//! According to Section 3.10.1, identifiers cannot start with digits.

use serde_kdl_macro::kdl;

fn main() {
    // This should fail: identifier cannot start with digit
    let _ = kdl! {
        9xyz "value"
    };
}
