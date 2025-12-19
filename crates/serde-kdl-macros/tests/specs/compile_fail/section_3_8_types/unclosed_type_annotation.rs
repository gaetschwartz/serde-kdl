use serde_kdl_macros::kdl;

fn main() {
    // Unclosed type annotation - missing closing parenthesis
    let _ = kdl! { node (u8 42 };
}
