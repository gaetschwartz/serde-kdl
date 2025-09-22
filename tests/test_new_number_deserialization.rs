use serde_kdl::{from_str, to_string};

#[test]
fn test_i128_roundtrip() {
    // Test i128 values within i64 range
    let i128_within_range = i64::MAX as i128;
    let kdl_string = to_string(&i128_within_range).unwrap();
    println!("Serialized i128: {}", kdl_string);

    let deserialized: i128 = from_str(&kdl_string).unwrap();
    assert_eq!(i128_within_range, deserialized);

    // Test negative values - use a value close to i64::MIN but not exactly i64::MIN
    // since the kdl crate 4.7 can't parse i64::MIN
    let i128_negative = (i64::MIN + 1) as i128;
    let kdl_string = to_string(&i128_negative).unwrap();
    println!("Serialized negative i128: {}", kdl_string);

    let deserialized: i128 = from_str(&kdl_string).unwrap();
    assert_eq!(i128_negative, deserialized);
}

#[test]
fn test_u128_roundtrip() {
    // Test u128 values within i64 range
    let u128_within_range = i64::MAX as u128;
    let kdl_string = to_string(&u128_within_range).unwrap();
    println!("Serialized u128: {}", kdl_string);

    let deserialized: u128 = from_str(&kdl_string).unwrap();
    assert_eq!(u128_within_range, deserialized);
}

#[test]
fn test_u64_roundtrip() {
    // Test u64 values within i64 range
    let u64_within_range = i64::MAX as u64;
    let kdl_string = to_string(&u64_within_range).unwrap();
    println!("Serialized u64: {}", kdl_string);

    let deserialized: u64 = from_str(&kdl_string).unwrap();
    assert_eq!(u64_within_range, deserialized);
}

#[test]
fn test_direct_kdl_parsing() {
    // Test direct KDL parsing with different integer types
    let test_cases = [
        ("root 42", 42i64),
        ("root -42", -42i64),
        ("root 9223372036854775807", i64::MAX), // i64::MAX
        ("root -9223372036854775807", i64::MIN + 1), // Close to i64::MIN but parseable
    ];

    for (kdl_str, expected) in &test_cases {
        println!("Testing: {}", kdl_str);

        // Test i64 deserialization
        let result_i64: i64 = from_str(kdl_str).unwrap();
        assert_eq!(result_i64, *expected);

        // Test i128 deserialization
        let result_i128: i128 = from_str(kdl_str).unwrap();
        assert_eq!(result_i128, *expected as i128);

        // Test u64 deserialization for positive values
        if *expected >= 0 {
            let result_u64: u64 = from_str(kdl_str).unwrap();
            assert_eq!(result_u64, *expected as u64);
        }

        // Test u128 deserialization for positive values
        if *expected >= 0 {
            let result_u128: u128 = from_str(kdl_str).unwrap();
            assert_eq!(result_u128, *expected as u128);
        }
    }
}