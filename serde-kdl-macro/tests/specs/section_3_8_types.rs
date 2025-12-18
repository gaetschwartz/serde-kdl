//! Tests for KDL Section 3.8: Type Annotation
//!
//! According to the KDL specification Section 3.8:
//! - Type annotations are a prefix to any Node Name (Section 3.2) or Value (Section 3.7)
//! - Written as `(` and `)` with a single String in it
//! - May contain whitespace after `(` and before `)`
//! - May be separated from target by whitespace
//! - Reserved type annotations for numbers without decimals (Section 3.8.1)
//! - Reserved type annotations for numbers with decimals (Section 3.8.2)
//! - Reserved type annotations for strings (Section 3.8.3)

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macro::kdl;

#[test]
fn test_value_type_annotations() {
    // Test type annotations on values (arguments and properties)
    // Covers: numeric types (i8, u32, f64), string types (email, url, uuid),
    // whitespace handling, and custom types
    let doc = kdl! {
        integers (i8)127 (u32)4294967295 (isize)123
        floats (f32)3.14 (f64)2.718281828 (decimal64)123.456
        strings (email)"test@example.com" (url)"https://example.com" (uuid)"550e8400-e29b-41d4-a716-446655440000"
        properties int=(i32)42 float=(f64)3.14 text=(regex)".*"
        whitespace ( u8 )123 (f32) 2.71 key = ( email ) "user@test.com"
        custom (MyType)"value" (custom-type)42 flag=(AppBoolean)#true
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    integers (i8)127 (u32)4294967295 (isize)123
    floats (f32)3.14 (f64)2.718281828 (decimal64)123.456
    strings (email)test@example.com (url)"https://example.com" (uuid)"550e8400-e29b-41d4-a716-446655440000"
    properties int=(i32)42 float=(f64)3.14 text=(regex).*
    whitespace (u8)123 (f32)2.71 key=(email)user@test.com
    custom (MyType)value (custom-type)42 flag=(AppBoolean)#true
    "#);
}

#[test]
fn test_node_name_type_annotations() {
    // Test type annotations on node names
    // Covers: node name annotations, spec examples, whitespace handling
    let doc = kdl! {
        (published)article title="Hello World"
        (contributor)person name="Foo McBar" age=(u8)30
        ( service ) database host=(hostname)"localhost" port=(u16)5432 {
            (table)users name=(regex)"[a-zA-Z]+"
            (table)posts title="Test"
        }
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    (published)article title="Hello World"
    (contributor)person name="Foo McBar" age=(u8)30
    (service)database host=(hostname)localhost port=(u16)5432 {
        (table)users name=(regex)"[a-zA-Z]+"
        (table)posts title=Test
    }
    "#);
}

#[test]
fn test_reserved_type_annotations_comprehensive() {
    // Test all reserved type annotations from KDL spec sections 3.8.1-3.8.3
    // Demonstrates comprehensive coverage of standard types
    let doc = kdl! {
        // Section 3.8.1: Numbers without decimals
        integers (i8)127 (i16)32767 (i32)2147483647 (i64)9223372036854775807
        unsigned (u8)255 (u16)65535 (u32)4294967295 (u64)18446744073709551615

        // Section 3.8.2: Numbers with decimals
        floats (f32)3.14 (f64)2.718281828
        decimals (decimal64)123.456 (decimal128)789.012345

        // Section 3.8.3: String types
        datetime (date-time)"2023-01-01T12:00:00Z" (time)"12:00:00" (date)"2023-01-01" (duration)"P1D"
        location (country-2)"US" (country-3)"USA" (country-subdivision)"US-CA"
        network (email)"test@example.com" (hostname)"example.com" (ipv4)"192.168.1.1" (ipv6)"2001:0db8:85a3::8a2e:0370:7334"
        urls (url)"https://example.com" (irl)"https://example.org" (hostname)"test.com"
        other (decimal)"123.456" (currency)"USD" (uuid)"550e8400-e29b-41d4-a716-446655440000" (regex)".*" (base64)"SGVsbG8gV29ybGQ="
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    integers (i8)127 (i16)32767 (i32)2147483647 (i64)9223372036854775807
    unsigned (u8)255 (u16)65535 (u32)4294967295 (u64)18446744073709551615
    floats (f32)3.14 (f64)2.718281828
    decimals (decimal64)123.456 (decimal128)789.012345
    datetime (date-time)"2023-01-01T12:00:00Z" (time)"12:00:00" (date)"2023-01-01" (duration)P1D
    location (country-2)US (country-3)USA (country-subdivision)US-CA
    network (email)test@example.com (hostname)example.com (ipv4)"192.168.1.1" (ipv6)"2001:0db8:85a3::8a2e:0370:7334"
    urls (url)"https://example.com" (irl)"https://example.org" (hostname)test.com
    other (decimal)"123.456" (currency)USD (uuid)"550e8400-e29b-41d4-a716-446655440000" (regex).* (base64)"SGVsbG8gV29ybGQ="
    "#);
}
