use crate::error::{Error, Result};
use kdl::KdlNode;
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use super::seq::SeqDeserializer;
use super::structs::StructDeserializer;
use super::node::NodeDeserializer;

// Enum deserializer
pub(crate) struct EnumDeserializer<'de> {
    variant: &'de str,
    node: &'de KdlNode,
}

impl<'de> EnumDeserializer<'de> {
    pub(crate) fn new(variant: &'de str, node: &'de KdlNode) -> Self {
        Self { variant, node }
    }
}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        use serde::de::value::StrDeserializer;
        let variant_value = seed.deserialize(StrDeserializer::<Error>::new(self.variant))?;
        let variant_de = VariantDeserializer::new(self.node);
        Ok((variant_value, variant_de))
    }
}

// Variant deserializer
pub(crate) struct VariantDeserializer<'de> {
    node: &'de KdlNode,
}

impl<'de> VariantDeserializer<'de> {
    pub(crate) fn new(node: &'de KdlNode) -> Self {
        Self { node }
    }
}

impl<'de> VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let de = NodeDeserializer::new(self.node);
        seed.deserialize(de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // First check if entries are directly in the node
        if !self.node.entries().is_empty() {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(children) = self.node.children() {
            // Look for a "tuple" child node with the values
            if let Some(tuple_node) = children
                .nodes()
                .iter()
                .find(|n| n.name().value() == "tuple")
            {
                let seq_de = SeqDeserializer::from_entries(tuple_node.entries());
                visitor.visit_seq(seq_de)
            } else {
                visitor.visit_seq(SeqDeserializer::from_entries(&[]))
            }
        } else {
            visitor.visit_seq(SeqDeserializer::from_entries(&[]))
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let struct_de = StructDeserializer::new(self.node);
        visitor.visit_map(struct_de)
    }
}
