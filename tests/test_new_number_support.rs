use serde_kdl::to_string;

#[test]
fn test_improved_u64_serialization() {
    // Test u64 values within i64 range - should work
    let u64_within_range = i64::MAX as u64;
    let result = to_string(&u64_within_range).unwrap();
    println!("u64 within range: {}", result);
    assert!(result.contains(&u64_within_range.to_string()));

    // Test u64 values beyond i64 range - should give better error message
    let u64_beyond_range = u64::MAX;
    match to_string(&u64_beyond_range) {
        Ok(_) => panic!("Expected error for u64::MAX"),
        Err(e) => {
            println!("u64::MAX error: {}", e);
            assert!(e
                .to_string()
                .contains("exceeds KDL's supported integer range"));
        }
    }
}

#[test]
fn test_i128_serialization() {
    // Test i128 values within i64 range - should work
    let i128_within_range = i64::MAX as i128;
    let result = to_string(&i128_within_range).unwrap();
    println!("i128 within range: {}", result);
    assert!(result.contains(&i128_within_range.to_string()));

    let i128_min_within_range = i64::MIN as i128;
    let result = to_string(&i128_min_within_range).unwrap();
    println!("i128 min within range: {}", result);
    assert!(result.contains(&i128_min_within_range.to_string()));

    // Test i128 values beyond i64 range - should give error message
    let i128_beyond_range = i128::MAX;
    match to_string(&i128_beyond_range) {
        Ok(_) => panic!("Expected error for i128::MAX"),
        Err(e) => {
            println!("i128::MAX error: {}", e);
            assert!(e
                .to_string()
                .contains("exceeds KDL's supported integer range"));
        }
    }
}

#[test]
fn test_u128_serialization() {
    // Test u128 values within i64 range - should work
    let u128_within_range = i64::MAX as u128;
    let result = to_string(&u128_within_range).unwrap();
    println!("u128 within range: {}", result);
    assert!(result.contains(&u128_within_range.to_string()));

    // Test u128 values beyond i64 range - should give error message
    let u128_beyond_range = u128::MAX;
    match to_string(&u128_beyond_range) {
        Ok(_) => panic!("Expected error for u128::MAX"),
        Err(e) => {
            println!("u128::MAX error: {}", e);
            assert!(e
                .to_string()
                .contains("exceeds KDL's supported integer range"));
        }
    }
}

#[test]
fn test_special_float_serialization() {
    // Test special float values
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    let normal = 2.5f64;

    let inf_result = to_string(&inf).unwrap();
    let neg_inf_result = to_string(&neg_inf).unwrap();
    let nan_result = to_string(&nan).unwrap();
    let normal_result = to_string(&normal).unwrap();

    println!("Infinity: {}", inf_result);
    println!("Negative infinity: {}", neg_inf_result);
    println!("NaN: {}", nan_result);
    println!("Normal float: {}", normal_result);

    // These should serialize without error (values will be represented as large numbers/0.0)
    // Note: f64::INFINITY and f64::NEG_INFINITY serialize as very large numbers
    // and f64::NAN serializes as 0.0 in kdl crate 4.7
    assert!(inf_result.contains("root"));
    assert!(neg_inf_result.contains("root"));
    assert!(nan_result.contains("0.0"));
    assert!(normal_result.contains("2.5"));
}
