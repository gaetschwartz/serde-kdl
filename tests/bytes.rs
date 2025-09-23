#[cfg(feature = "bytes")]
mod bytes_tests {
    use serde::{Deserialize, Serialize};
    use serde_kdl::{from_str, to_string};

    // Test that Vec<u8> without #[serde(with)] now serializes as hex (new default behavior)
    #[test]
    fn test_vec_u8_without_attribute_serializes_as_hex() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct BytesAsHex {
            name: String,
            data: Vec<u8>,
        }

        let test_data = BytesAsHex {
            name: "hex_default".to_string(),
            data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello"
        };

        let kdl_string = to_string(&test_data).unwrap();
        println!("Serialized: {}", kdl_string);
        // Should serialize as hex string by default when bytes feature enabled
        assert!(kdl_string.contains("48656c6c6f"));

        let deserialized: BytesAsHex = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_both_approaches_produce_same_result() {
        // Both explicit #[serde(with)] and default behavior should produce same result

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithExplicitHex {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithDefaultHex {
            data: Vec<u8>,
        }

        let test_bytes = vec![0x12, 0x34, 0x56];

        let with_explicit = WithExplicitHex {
            data: test_bytes.clone(),
        };
        let with_default = WithDefaultHex {
            data: test_bytes.clone(),
        };

        let explicit_kdl = to_string(&with_explicit).unwrap();
        let default_kdl = to_string(&with_default).unwrap();

        // Both should produce hex string
        assert!(explicit_kdl.contains("123456"));
        assert!(default_kdl.contains("123456"));

        // Both should deserialize correctly
        let explicit_deserialized: WithExplicitHex = from_str(&explicit_kdl).unwrap();
        let default_deserialized: WithDefaultHex = from_str(&default_kdl).unwrap();

        assert_eq!(with_explicit, explicit_deserialized);
        assert_eq!(with_default, default_deserialized);
    }

    #[test]
    fn test_empty_vec_u8_default_behavior() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct EmptyBytes {
            data: Vec<u8>,
        }

        let test_data = EmptyBytes { data: vec![] };

        let kdl_string = to_string(&test_data).unwrap();
        let deserialized: EmptyBytes = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_optional_vec_u8_default_behavior() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct OptionalBytes {
            data: Option<Vec<u8>>,
        }

        // Test with Some
        let with_data = OptionalBytes {
            data: Some(vec![0xff, 0x00, 0xaa]),
        };

        let kdl_string = to_string(&with_data).unwrap();
        let deserialized: OptionalBytes = from_str(&kdl_string).unwrap();
        assert_eq!(with_data, deserialized);

        // Test with None
        let without_data = OptionalBytes { data: None };

        let kdl_string = to_string(&without_data).unwrap();
        let deserialized: OptionalBytes = from_str(&kdl_string).unwrap();
        assert_eq!(without_data, deserialized);
    }

    // Test hex_serde module functionality
    #[test]
    fn test_hex_serde_basic() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithHexBytes {
            name: String,
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let test_data = WithHexBytes {
            name: "test".to_string(),
            data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello"
        };

        let kdl_string = to_string(&test_data).unwrap();
        assert!(kdl_string.contains("48656c6c6f"));

        let deserialized: WithHexBytes = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_hex_serde_empty() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithEmptyBytes {
            name: String,
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let test_data = WithEmptyBytes {
            name: "empty".to_string(),
            data: vec![],
        };

        let kdl_string = to_string(&test_data).unwrap();
        let deserialized: WithEmptyBytes = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_hex_serde_all_bytes() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithAllBytes {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let test_data = WithAllBytes {
            data: (0..=255).collect(),
        };

        let kdl_string = to_string(&test_data).unwrap();
        let deserialized: WithAllBytes = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_hex_serde_slice() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithSliceBytes {
            name: String,
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let data = [0xde, 0xad, 0xbe, 0xef];
        let test_data = WithSliceBytes {
            name: "slice".to_string(),
            data: data.to_vec(),
        };

        let kdl_string = to_string(&test_data).unwrap();
        assert!(kdl_string.contains("deadbeef"));

        let deserialized: WithSliceBytes = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    // Note: Nested struct serialization with bytes is currently limited
    // by serde-kdl's struct serialization implementation
    #[test]
    fn test_multiple_hex_fields_in_struct() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct MultipleHexFields {
            name: String,
            #[serde(with = "serde_kdl::hex_serde")]
            data1: Vec<u8>,
            #[serde(with = "serde_kdl::hex_serde")]
            data2: Vec<u8>,
        }

        let test_data = MultipleHexFields {
            name: "multi".to_string(),
            data1: vec![0x12, 0x34],
            data2: vec![0x56, 0x78],
        };

        let kdl_string = to_string(&test_data).unwrap();
        let deserialized: MultipleHexFields = from_str(&kdl_string).unwrap();
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_optional_hex_bytes() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct OptionalBytes {
            name: String,
            #[serde(with = "serde_kdl::hex_serde::option")]
            data: Option<Vec<u8>>,
        }

        // Test with Some
        let with_data = OptionalBytes {
            name: "with_data".to_string(),
            data: Some(vec![0xff, 0x00, 0xaa]),
        };

        let kdl_string = to_string(&with_data).unwrap();
        let deserialized: OptionalBytes = from_str(&kdl_string).unwrap();
        assert_eq!(with_data, deserialized);
    }

    #[test]
    fn test_manual_hex_deserialization() {
        // Test that we can deserialize manually constructed hex strings when using bytes feature
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct HexData {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let kdl_input = r#"HexData data="deadbeef""#;
        let result: HexData = from_str(kdl_input).unwrap();
        assert_eq!(result.data, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_uppercase_hex_deserialization() {
        // Test that uppercase hex works too
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct HexData {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let kdl_input = r#"HexData data="DEADBEEF""#;
        let result: HexData = from_str(kdl_input).unwrap();
        assert_eq!(result.data, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_mixed_case_hex_deserialization() {
        // Test mixed case
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct HexData {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let kdl_input = r#"HexData data="DeAdBeEf""#;
        let result: HexData = from_str(kdl_input).unwrap();
        assert_eq!(result.data, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_invalid_hex_string() {
        // Test that invalid hex strings produce errors
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct HexData {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        let kdl_input = r#"HexData data="invalid""#;
        let result: Result<HexData, _> = from_str(kdl_input);
        assert!(result.is_err());

        // Test odd length hex string
        let kdl_input = r#"HexData data="abc""#;
        let result: Result<HexData, _> = from_str(kdl_input);
        assert!(result.is_err());
    }

    #[test]
    fn test_bytes_roundtrip_consistency() {
        // Test that serialization and deserialization are consistent using hex_serde
        use rand::Rng;
        let mut rng = rand::thread_rng();

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct ByteData {
            #[serde(with = "serde_kdl::hex_serde")]
            data: Vec<u8>,
        }

        for _ in 0..100 {
            let len = rng.gen_range(0..=256);
            let bytes: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

            let byte_data = ByteData {
                data: bytes.clone(),
            };
            let kdl_string = to_string(&byte_data).unwrap();
            let deserialized: ByteData = from_str(&kdl_string).unwrap();

            assert_eq!(
                byte_data, deserialized,
                "Roundtrip failed for bytes: {:?}",
                bytes
            );
        }
    }
}

#[cfg(not(feature = "bytes"))]
mod no_bytes_tests {
    use serde::Serializer as SerializerTrait;
    use serde_kdl::{to_string, Serializer};

    #[test]
    fn test_bytes_serializer_not_supported_without_feature() {
        // Test that serialize_bytes directly returns an error when bytes feature is disabled
        let mut serializer = Serializer::new();
        let bytes = &[0x01u8, 0x02u8, 0x03u8];
        let result = (&mut serializer).serialize_bytes(bytes);
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("byte arrays"));
    }

    #[test]
    fn test_vec_u8_still_works_as_sequence() {
        // Vec<u8> should still work as a regular sequence even without bytes feature
        let bytes = vec![0x01, 0x02, 0x03];
        let result = to_string(&bytes);
        assert!(result.is_ok());

        let kdl_string = result.unwrap();
        // Should serialize as a sequence, not as hex
        assert!(kdl_string.contains("1 2 3"));
    }
}
