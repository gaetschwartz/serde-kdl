//! Test that booleans cannot be used as property keys
//! Section 3.7 specifies: Only String values may be used as Property keys

use serde_kdl_macros::kdl;

fn main() {
    // Booleans are not valid property keys
    let _ = kdl! { node #true="value" };
}
