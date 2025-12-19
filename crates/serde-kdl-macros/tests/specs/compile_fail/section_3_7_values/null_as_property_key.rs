//! Test that null cannot be used as a property key
//! Section 3.7 specifies: Only String values may be used as Property keys

use serde_kdl_macros::kdl;

fn main() {
    // Null is not a valid property key
    let _ = kdl! { node #null="value" };
}
