use super::node::NodeDeserializer;
use crate::error::{Error, Result};
use kdl::KdlNode;
use serde::de::{DeserializeSeed, MapAccess};

// Struct deserializer that only handles child nodes (no properties)
pub(crate) struct StructDeserializer<'de> {
    node: &'de KdlNode,
    child_index: usize,
    current_node: Option<&'de KdlNode>,
}

impl<'de> StructDeserializer<'de> {
    pub(crate) fn new(node: &'de KdlNode) -> Self {
        Self {
            node,
            child_index: 0,
            current_node: None,
        }
    }
}

impl<'de> MapAccess<'de> for StructDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Only look for child nodes - no properties
        if let Some(children) = self.node.children() {
            if self.child_index < children.nodes().len() {
                let child = &children.nodes()[self.child_index];
                self.child_index += 1;

                self.current_node = Some(child);
                use serde::de::value::StrDeserializer;
                return seed
                    .deserialize(StrDeserializer::<Error>::new(child.name().value()))
                    .map(Some);
            }
        }

        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(node) = self.current_node.take() {
            let de = NodeDeserializer::new(node);
            seed.deserialize(de)
        } else {
            Err(Error::Serde("no current node for value".to_string()))
        }
    }
}

// Root struct deserializer - treats document nodes as struct fields
pub(crate) struct RootStructDeserializer<'de> {
    nodes: std::slice::Iter<'de, KdlNode>,
    current_node: Option<&'de KdlNode>,
}

impl<'de> RootStructDeserializer<'de> {
    pub(crate) fn new(nodes: &'de [KdlNode]) -> Self {
        Self {
            nodes: nodes.iter(),
            current_node: None,
        }
    }
}

impl<'de> MapAccess<'de> for RootStructDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(node) = self.nodes.next() {
            self.current_node = Some(node);
            use serde::de::value::StrDeserializer;
            seed.deserialize(StrDeserializer::<Error>::new(node.name().value()))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(node) = self.current_node.take() {
            let de = NodeDeserializer::new(node);
            seed.deserialize(de)
        } else {
            Err(Error::Serde("no current node for value".to_string()))
        }
    }
}
