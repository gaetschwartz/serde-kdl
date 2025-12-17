// Null keyword is case-sensitive: must be lowercase #null, not #Null
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! {
        node #Null
    };
}
