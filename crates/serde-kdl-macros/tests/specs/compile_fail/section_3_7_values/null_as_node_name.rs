//! Test that null cannot be used as a node name
//! Section 3.7 specifies: Only String values may be used as Node names

use serde_kdl_macros::kdl;

fn main() {
    // Null is not a valid node name
    let _ = kdl! { #null "arg" };
}
