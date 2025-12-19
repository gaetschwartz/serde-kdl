use super::entry::EntryDeserializer;
use super::node::NodeDeserializer;
use crate::error::{Error, Result};
use kdl::KdlNode;
use serde::de::{DeserializeSeed, SeqAccess};

// Sequence deserializer
pub(crate) struct SeqDeserializer<'de> {
    entries: std::slice::Iter<'de, kdl::KdlEntry>,
    children: std::slice::Iter<'de, KdlNode>,
    mode: SeqMode,
}

pub(crate) enum SeqMode {
    Entries,
    Children,
}

impl<'de> SeqDeserializer<'de> {
    pub(crate) fn from_entries(entries: &'de [kdl::KdlEntry]) -> Self {
        Self {
            entries: entries.iter(),
            children: [].iter(),
            mode: SeqMode::Entries,
        }
    }

    pub(crate) fn from_children(children: &'de [KdlNode]) -> Self {
        Self {
            entries: [].iter(),
            children: children.iter(),
            mode: SeqMode::Children,
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.mode {
            SeqMode::Entries => {
                if let Some(entry) = self.entries.next() {
                    // Use a specialized deserializer that can handle both simple values and enums
                    let de = EntryDeserializer::new(entry);
                    seed.deserialize(de).map(Some)
                } else {
                    Ok(None)
                }
            }
            SeqMode::Children => {
                if let Some(child) = self.children.next() {
                    // Check if this is a "-" wrapper node
                    if child.name().value() == "-" {
                        // For "-" nodes, deserialize from the node's content
                        let de = NodeDeserializer::new(child);
                        seed.deserialize(de).map(Some)
                    } else {
                        // Regular child node
                        let de = NodeDeserializer::new(child);
                        seed.deserialize(de).map(Some)
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }
}
