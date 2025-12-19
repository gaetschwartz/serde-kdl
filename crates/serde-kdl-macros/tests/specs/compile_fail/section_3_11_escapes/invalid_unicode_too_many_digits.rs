// Test that Unicode escapes with more than 6 hex digits are rejected
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! { node "\u{1234567}" }; // Too many hex digits (7, max is 6)
}
