// Test that Unicode escapes with non-hex characters are rejected
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! { node "\u{GGGG}" }; // Invalid hex character 'G'
}
