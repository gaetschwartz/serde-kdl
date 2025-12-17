//! Test that booleans cannot be used as node names
//! Section 3.7 specifies: Only String values may be used as Node names

use serde_kdl_macro::kdl;

fn main() {
    // Booleans are not valid node names
    let _ = kdl! { #true "arg" };
}
