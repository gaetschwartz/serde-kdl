use super::node::NodeDeserializer;
use crate::error::{Error, Result};
use kdl::KdlNode;
use serde::de::{DeserializeSeed, MapAccess};

// Map deserializer
pub(crate) struct MapDeserializer<'de> {
    children: std::slice::Iter<'de, KdlNode>,
    current_key: Option<&'de str>,
    current_node: Option<&'de KdlNode>,
}

impl<'de> MapDeserializer<'de> {
    pub(crate) fn new(children: &'de [KdlNode]) -> Self {
        Self {
            children: children.iter(),
            current_key: None,
            current_node: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(child) = self.children.next() {
            self.current_key = Some(child.name().value());
            self.current_node = Some(child);
            use serde::de::value::StrDeserializer;
            seed.deserialize(StrDeserializer::<Error>::new(child.name().value()))
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
