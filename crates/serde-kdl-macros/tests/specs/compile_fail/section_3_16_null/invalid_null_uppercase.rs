// Null keyword is case-sensitive: must be lowercase #null, not #NULL
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node #NULL
    };
}
