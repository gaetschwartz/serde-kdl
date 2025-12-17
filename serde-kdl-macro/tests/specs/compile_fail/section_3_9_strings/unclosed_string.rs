//! Test that unclosed strings fail to compile
//!
//! This should produce an error because the string is not properly closed.

use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! {
        node "unclosed
    };
}
