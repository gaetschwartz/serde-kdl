//! Test that bare identifiers cannot start with a digit
//!
//! According to Section 3.10.1, identifiers cannot start with digits.
//! Bare identifiers starting with digits should fail at compile time.

use serde_kdl_macros::kdl;

fn main() {
    // This should fail: identifier cannot start with digit
    let _ = kdl! {
        1abc "value"
    };
}
