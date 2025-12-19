use serde_kdl_macros::kdl;

fn main() {
    // Invalid: hex prefix without digits
    let _ = kdl! { node 0x };
}
