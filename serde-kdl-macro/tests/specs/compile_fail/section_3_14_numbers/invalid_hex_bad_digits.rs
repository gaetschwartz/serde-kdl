use serde_kdl_macro::kdl;

fn main() {
    // Invalid: hex with non-hex digit
    let _ = kdl! { node 0xGHI };
}
