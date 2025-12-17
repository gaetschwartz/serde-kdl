//! Test that numbers cannot be used as property keys
//! Section 3.7 specifies: Only String values may be used as Property keys

use serde_kdl_macro::kdl;

fn main() {
    // Numbers are not valid property keys
    let _ = kdl! { node 42="value" };
}
