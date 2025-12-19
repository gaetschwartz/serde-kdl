use crate::error::{Error, Result};
use kdl::{KdlEntry, KdlNode, KdlValue};
use serde::de::{DeserializeSeed, MapAccess};
use super::node::NodeDeserializer;
use super::entry::EntryDeserializer;

// Struct deserializer that handles both properties and children
pub(crate) struct StructDeserializer<'de> {
    node: &'de KdlNode,
    entry_index: usize,
    child_index: usize,
    current_field: Option<&'de str>,
    current_value: Option<FieldValue<'de>>,
}

#[derive(Clone)]
pub(crate) enum FieldValue<'de> {
    Property(&'de KdlValue),
    ChildNode(&'de KdlNode),
}

impl<'de> StructDeserializer<'de> {
    pub(crate) fn new(node: &'de KdlNode) -> Self {
        Self {
            node,
            entry_index: 0,
            child_index: 0,
            current_field: None,
            current_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for StructDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // First, look for property entries
        while self.entry_index < self.node.entries().len() {
            let entry = &self.node.entries()[self.entry_index];
            self.entry_index += 1;

            if let Some(name) = entry.name() {
                // This is a property entry
                self.current_field = Some(name.value());
                self.current_value = Some(FieldValue::Property(entry.value()));
                use serde::de::value::StrDeserializer;
                return seed
                    .deserialize(StrDeserializer::<Error>::new(name.value()))
                    .map(Some);
            }
            // Skip non-property entries
        }

        // Then look for child nodes
        if let Some(children) = self.node.children() {
            if self.child_index < children.nodes().len() {
                let child = &children.nodes()[self.child_index];
                self.child_index += 1;

                self.current_field = Some(child.name().value());
                self.current_value = Some(FieldValue::ChildNode(child));
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
        if let Some(field_value) = self.current_value.take() {
            match field_value {
                FieldValue::Property(value) => {
                    // Create a temporary entry for the property value to handle enums
                    let entry = KdlEntry::new(value.clone());
                    let de = EntryDeserializer::new_owned(entry);
                    seed.deserialize(de)
                }
                FieldValue::ChildNode(node) => {
                    let de = NodeDeserializer::new(node);
                    seed.deserialize(de)
                }
            }
        } else {
            Err(Error::Serde("no current value for field".to_string()))
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
