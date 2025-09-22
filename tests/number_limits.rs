use kdl::{KdlValue, KdlDocument};

#[test]
fn test_kdl_value_base10_range() {
    // Test if Base10 can accept i128
    let val1 = KdlValue::Base10(i64::MAX);
    let val2 = KdlValue::Base10(i64::MIN);

    // Test if the KdlValue constructor can take i128
    // This will fail to compile if Base10 only accepts i64
    // let val3 = KdlValue::Base10(i128::MAX); // Confirmed: only accepts i64

    println!("Base10 with i64::MAX: {:?}", val1);
    println!("Base10 with i64::MIN: {:?}", val2);

    // Test if newer variants exist
    // let val_integer = KdlValue::Integer(i128::MAX); // Not available in kdl 4.7
    // let val_float = KdlValue::Float(3.14f64); // Not available in kdl 4.7
}

#[test]
fn test_kdl_parse_large_numbers() {
    // Test what happens when we parse large numbers with KDL
    let test_cases = [
        "root 9223372036854775807",  // i64::MAX
        "root -9223372036854775808", // i64::MIN
        "root 9223372036854775808",  // i64::MAX + 1
        "root 18446744073709551615", // u64::MAX
        "root 170141183460469231731687303715884105727", // i128::MAX
    ];

    for case in &test_cases {
        println!("Testing: {}", case);
        match case.parse::<KdlDocument>() {
            Ok(doc) => {
                if let Some(node) = doc.nodes().first() {
                    if let Some(entry) = node.entries().first() {
                        println!("  Parsed successfully: {:?}", entry.value());
                    } else {
                        println!("  No entry found");
                    }
                } else {
                    println!("  No node found");
                }
            }
            Err(e) => println!("  Parse error: {}", e),
        }
    }
}

#[test]
fn test_kdl_parse_special_floats() {
    // Test parsing of special float values
    let test_cases = [
        "root #inf",
        "root #-inf",
        "root #nan",
        "root 3.14",
        "root -2.5",
    ];

    for case in &test_cases {
        println!("Testing: {}", case);
        match case.parse::<KdlDocument>() {
            Ok(doc) => {
                if let Some(node) = doc.nodes().first() {
                    if let Some(entry) = node.entries().first() {
                        println!("  Parsed successfully: {:?}", entry.value());
                    } else {
                        println!("  No entry found");
                    }
                } else {
                    println!("  No node found");
                }
            }
            Err(e) => println!("  Parse error: {}", e),
        }
    }
}

#[test]
fn test_current_serde_limitation() {
    use serde_kdl::to_string;

    // Test current limitations
    let u64_max = u64::MAX;
    let u64_over_i64_max = (i64::MAX as u64) + 1;
    let u64_within_i64_max = i64::MAX as u64;

    println!("Testing u64::MAX: {}", u64_max);
    match to_string(&u64_max) {
        Ok(s) => println!("u64::MAX serialized: {}", s),
        Err(e) => println!("u64::MAX failed: {}", e),
    }

    println!("Testing u64 > i64::MAX: {}", u64_over_i64_max);
    match to_string(&u64_over_i64_max) {
        Ok(s) => println!("u64 > i64::MAX serialized: {}", s),
        Err(e) => println!("u64 > i64::MAX failed: {}", e),
    }

    println!("Testing u64 <= i64::MAX: {}", u64_within_i64_max);
    match to_string(&u64_within_i64_max) {
        Ok(s) => println!("u64 <= i64::MAX serialized: {}", s),
        Err(e) => println!("u64 <= i64::MAX failed: {}", e),
    }
}