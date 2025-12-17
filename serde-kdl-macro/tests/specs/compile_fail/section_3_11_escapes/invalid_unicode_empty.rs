// Test that empty Unicode escapes are rejected
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! { node "\u{}" }; // Empty Unicode escape sequence
}
