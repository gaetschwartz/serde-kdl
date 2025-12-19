use crate::error::{Error, Result};
use kdl::{KdlNode, KdlValue};
use serde::de::Visitor;
use serde::Deserializer as DeserializerTrait;
use super::seq::SeqDeserializer;
#[cfg(feature = "bytes")]
use super::seq::BytesSeqDeserializer;
use super::map::MapDeserializer;
use super::structs::StructDeserializer;
use super::variants::EnumDeserializer;

/// A deserializer that works directly with a single `KdlNode`
pub(crate) struct NodeDeserializer<'de> {
    node: &'de KdlNode,
}

impl<'de> NodeDeserializer<'de> {
    pub(crate) fn new(node: &'de KdlNode) -> Self {
        Self { node }
    }

    fn deserialize_from_value<V>(&self, value: &'de KdlValue, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match value {
            KdlValue::String(s) => visitor.visit_str(s),
            KdlValue::Integer(i) => {
                // serde visitors for most numeric types only accept i64, not i128
                if let Ok(v) = i64::try_from(*i) {
                    visitor.visit_i64(v)
                } else {
                    visitor.visit_i128(*i)
                }
            }
            KdlValue::Float(f) => visitor.visit_f64(*f),
            KdlValue::Bool(b) => visitor.visit_bool(*b),
            KdlValue::Null => visitor.visit_unit(),
        }
    }
}

impl<'de> DeserializerTrait<'de> for NodeDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If the node has multiple entries, treat as a sequence
        if self.node.entries().len() > 1 {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(entry) = self.node.entries().first() {
            // Single entry - deserialize the value
            self.deserialize_from_value(entry.value(), visitor)
        } else if let Some(children) = self.node.children() {
            if children.nodes().is_empty() {
                visitor.visit_unit()
            } else {
                // If the node has children, treat it as a map
                let map_de = MapDeserializer::new(children.nodes());
                visitor.visit_map(map_de)
            }
        } else {
            // Empty node - treat as unit
            visitor.visit_unit()
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Check for single string entry that might be hex bytes
        #[cfg(feature = "bytes")]
        {
            if self.node.entries().len() == 1 {
                if let Some(entry) = self.node.entries().first() {
                    if entry.name().is_none() {
                        // Not a property
                        if let KdlValue::String(s) = entry.value() {
                            // Try to decode as hex - if successful, use bytes deserializer
                            if let Ok(bytes) = crate::hex::decode_hex(s) {
                                return visitor.visit_seq(BytesSeqDeserializer::new(bytes));
                            }
                        }
                    }
                }
            }
        }

        if !self.node.entries().is_empty() {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(children) = self.node.children() {
            if children.nodes().is_empty() {
                visitor.visit_seq(SeqDeserializer::from_entries(&[]))
            } else {
                let seq_de = SeqDeserializer::from_children(children.nodes());
                visitor.visit_seq(seq_de)
            }
        } else {
            visitor.visit_seq(SeqDeserializer::from_entries(&[]))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Create a struct deserializer from the node's children
        let struct_de = StructDeserializer::new(self.node);
        visitor.visit_map(struct_de)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            if let Some(entry) = self.node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) => {
                        let bytes = crate::hex::decode_hex(s)?;
                        visitor.visit_bytes(&bytes)
                    }
                    _ => Err(Error::UnsupportedType(
                        "bytes must be represented as hex strings".to_string(),
                    )),
                }
            } else {
                Err(Error::UnsupportedType(
                    "no value found for bytes".to_string(),
                ))
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            if let Some(entry) = self.node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) => {
                        let bytes = crate::hex::decode_hex(s)?;
                        visitor.visit_byte_buf(bytes)
                    }
                    _ => Err(Error::UnsupportedType(
                        "bytes must be represented as hex strings".to_string(),
                    )),
                }
            } else {
                Err(Error::UnsupportedType(
                    "no value found for bytes".to_string(),
                ))
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let node_name = self.node.name().value();

        // If node name is a known variant, use it
        if variants.contains(&node_name) {
            return visitor.visit_enum(EnumDeserializer::new(node_name, self.node));
        }

        // Otherwise check if the node has a single string value - use it as variant name
        // This handles the transparent format: `field_name VariantName`
        if self.node.entries().len() == 1 {
            if let Some(entry) = self.node.entries().first() {
                if entry.name().is_none() {
                    if let KdlValue::String(s) = entry.value() {
                        use serde::de::value::StrDeserializer;
                        return visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()));
                    }
                }
            }
        }

        // Fallback to node name as variant
        visitor.visit_enum(EnumDeserializer::new(node_name, self.node))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Check if the first entry is Null - that means None
        if let Some(entry) = self.node.entries().first() {
            if matches!(entry.value(), KdlValue::Null) {
                return visitor.visit_none();
            }
        }

        // If node has entries or children, treat as Some
        let has_content = !self.node.entries().is_empty()
            || self.node.children().is_some_and(|c| !c.nodes().is_empty());

        if has_content {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        unit unit_struct newtype_struct tuple
        tuple_struct map identifier ignored_any
    }
}
