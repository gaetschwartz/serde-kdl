#[test]
fn debug_null_parsing() {
    use serde_kdl_macro::kdl_impl2;
    
    // Test simple null parsing first
    let input_str = "node null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    if let Err(e) = &result {
        println!("Error parsing simple null: {}", e);
    } else {
        println!("Simple null parsing: OK");
    }
    
    // Test type-annotated null
    let input_str = "node (custom)null";
    let input: proc_macro2::TokenStream = input_str.parse().unwrap();
    let result = kdl_impl2(input);
    if let Err(e) = &result {
        println!("Error parsing (custom)null: {}", e);
    } else {
        println!("Type-annotated null parsing: OK");
    }
}
