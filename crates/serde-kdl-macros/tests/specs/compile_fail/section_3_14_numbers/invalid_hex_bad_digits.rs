use serde_kdl_macros::kdl;

fn main() {
    // Invalid: hex with non-hex digit
    let _ = kdl! { node 0xGHI };
}
