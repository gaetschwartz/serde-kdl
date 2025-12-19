use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::ser::SerializeSeq;
use super::Serializer;

// Sequence serializer
pub struct SerializeSeqImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) items: Vec<KdlValue>,
    pub(crate) child_nodes: Vec<KdlNode>,
    #[cfg(feature = "bytes")]
    pub(crate) bytes_candidate: Option<Vec<u8>>,
}

impl SerializeSeq for SerializeSeqImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Create a temporary serializer for the item
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            // Check if this is a complex structure (has multiple entries, properties, or children)
            // Also consider enum variants (nodes with meaningful names) as complex
            let is_complex = node.entries().len() > 1
                || !node.entries().iter().all(|e| e.name().is_none()) // has properties
                || node.children().is_some_and(|c| !c.nodes().is_empty()) // has children
                || node.name().value() != crate::DEFAULT_NODE_NAME; // enum variants or other meaningful node names

            if is_complex {
                // Complex structure - store as child node
                self.child_nodes.push(node.clone());
                #[cfg(feature = "bytes")]
                {
                    // Complex elements mean this can't be a simple byte array
                    self.bytes_candidate = None;
                }
            } else if let Some(entry) = node.entries().first() {
                // Simple value - store in items for inline representation
                let value = entry.value().clone();

                #[cfg(feature = "bytes")]
                {
                    // Check if this value could be a byte (u8 in range 0-255)
                    if let Some(ref mut bytes) = self.bytes_candidate {
                        if let KdlValue::Integer(i) = &value {
                            if *i >= 0 && *i <= 255 {
                                bytes.push(*i as u8);
                            } else {
                                // Value out of u8 range, not a byte array
                                self.bytes_candidate = None;
                            }
                        } else {
                            // Non-integer value, not a byte array
                            self.bytes_candidate = None;
                        }
                    }
                }

                self.items.push(value);
            } else {
                // Node without value - create a null
                self.items.push(KdlValue::Null);
                #[cfg(feature = "bytes")]
                {
                    // Null values mean this can't be a simple byte array
                    self.bytes_candidate = None;
                }
            }
        } else {
            self.items.push(KdlValue::Null);
            #[cfg(feature = "bytes")]
            {
                // Null values mean this can't be a simple byte array
                self.bytes_candidate = None;
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);

        #[cfg(feature = "bytes")]
        {
            // Check if this sequence is a byte array that should be serialized as hex
            if let Some(bytes) = self.bytes_candidate {
                if !bytes.is_empty()
                    && self.child_nodes.is_empty()
                    && self.items.len() == bytes.len()
                {
                    // This looks like a byte array - serialize as hex string instead
                    let hex_string = crate::hex::encode_hex(&bytes);
                    node.entries_mut()
                        .push(KdlEntry::new(KdlValue::String(hex_string)));
                    self.ser.current_node = Some(node);
                    return Ok(());
                }
            }
        }

        // Add simple values as entries
        for item in self.items {
            node.entries_mut().push(KdlEntry::new(item));
        }

        // Add complex structures as children
        if !self.child_nodes.is_empty() {
            let mut child_doc = KdlDocument::new();
            for child in self.child_nodes {
                child_doc.nodes_mut().push(child);
            }
            *node.children_mut() = Some(child_doc);
        }

        self.ser.current_node = Some(node);
        Ok(())
    }
}
