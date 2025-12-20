use super::map::MapDeserializer;
use super::seq::SeqDeserializer;
use super::structs::StructDeserializer;
use super::variants::EnumDeserializer;
use crate::error::{Error, Result};
use kdl::{KdlNode, KdlValue};
use serde::Deserializer as DeserializerTrait;
use serde::de::Visitor;

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
                // Check if all children are sequence items (named "-")
                let is_sequence = children
                    .nodes()
                    .iter()
                    .all(|n| n.name().value() == crate::DEFAULT_NODE_NAME);

                if is_sequence {
                    // Sequence with "-" wrapper nodes
                    let seq_de = SeqDeserializer::from_children(children.nodes());
                    visitor.visit_seq(seq_de)
                } else {
                    // Struct/map - children are named fields
                    let map_de = MapDeserializer::new(children.nodes());
                    visitor.visit_map(map_de)
                }
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
        // Sequences are in children with "-" nodes, or from entries for tuples
        if !self.node.entries().is_empty() {
            // Tuple-like sequence from entries
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(children) = self.node.children() {
            if children.nodes().is_empty() {
                // Empty collection
                visitor.visit_seq(SeqDeserializer::from_entries(&[]))
            } else {
                // Sequence from children (may include "-" wrapper nodes)
                let seq_de = SeqDeserializer::from_children(children.nodes());
                visitor.visit_seq(seq_de)
            }
        } else {
            // Empty sequence
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
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Check for type annotation (e.g., (VariantName)field_name)
        if let Some(ty) = self.node.ty() {
            let variant_name = ty.value();
            return visitor.visit_enum(EnumDeserializer::new(variant_name, self.node));
        }

        // Check if the node has a single string value - use it as variant name
        // This handles unit enums serialized as strings
        if self.node.entries().len() == 1
            && let Some(entry) = self.node.entries().first()
            && entry.name().is_none()
            && let KdlValue::String(s) = entry.value()
        {
            use serde::de::value::StrDeserializer;
            return visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()));
        }

        // Use node name as variant for externally tagged enums
        // This handles: path "./" -> Enum::Path("./")
        let variant_name = self.node.name().value();
        visitor.visit_enum(EnumDeserializer::new(variant_name, self.node))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Check if node has a single #null argument -> None
        if self.node.entries().len() == 1
            && let Some(entry) = self.node.entries().first()
            && matches!(entry.value(), KdlValue::Null)
        {
            return visitor.visit_none();
        }

        // If node has entries, children, or a type annotation, treat as Some
        let has_content = !self.node.entries().is_empty()
            || self.node.children().is_some_and(|c| !c.nodes().is_empty())
            || self.node.ty().is_some(); // Type annotation means it's an enum value

        if has_content {
            visitor.visit_some(self)
        } else {
            // Empty node means None
            visitor.visit_none()
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        unit unit_struct newtype_struct tuple
        tuple_struct map identifier ignored_any
    }
}
