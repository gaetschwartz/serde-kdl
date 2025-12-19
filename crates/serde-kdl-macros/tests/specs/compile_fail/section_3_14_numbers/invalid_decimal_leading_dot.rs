use serde_kdl_macros::kdl;

fn main() {
    // Invalid: decimal number starting with dot
    let _ = kdl! { node .5 };
}
