use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        "struct" #true;
        typed (opt)"value"
    };
}
