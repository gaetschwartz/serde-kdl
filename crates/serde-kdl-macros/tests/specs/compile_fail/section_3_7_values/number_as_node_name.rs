//! Test that numbers cannot be used as node names
//! Section 3.7 specifies: Only String values may be used as Node names

use serde_kdl_macros::kdl;

fn main() {
    // Numbers are not valid node names
    let _ = kdl! { 42 "arg" };
}
