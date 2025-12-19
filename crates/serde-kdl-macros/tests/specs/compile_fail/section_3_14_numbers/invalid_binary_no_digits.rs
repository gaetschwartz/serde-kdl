use serde_kdl_macros::kdl;

fn main() {
    // Invalid: binary prefix without digits
    let _ = kdl! { node 0b };
}
