use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode};
use serde::ser::SerializeSeq;
use super::Serializer;

// Sequence serializer
pub struct SerializeSeqImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) child_nodes: Vec<KdlNode>,
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
            // Create a new node with "-" as the name
            let mut wrapper_node = KdlNode::new("-");

            // Copy all entries (arguments) from the serialized value
            for entry in node.entries() {
                wrapper_node.entries_mut().push(entry.clone());
            }

            // Copy children if any (for complex structures)
            if let Some(node_children) = node.children() {
                if !node_children.nodes().is_empty() {
                    *wrapper_node.children_mut() = Some(node_children.clone());
                }
            }

            // Copy type annotation if present (for enum variants)
            if let Some(ty) = node.ty() {
                wrapper_node.set_ty(ty.clone());
            }

            self.child_nodes.push(wrapper_node);
        } else {
            // Empty document - create a "-" node with null value
            let mut wrapper_node = KdlNode::new("-");
            wrapper_node.entries_mut().push(KdlEntry::new(kdl::KdlValue::Null));
            self.child_nodes.push(wrapper_node);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Create the parent node with "-" as the name
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);

        // If we have child nodes, add them in a children block
        // Even if empty, we still create an empty children block for empty sequences
        let mut child_doc = KdlDocument::new();
        for child in self.child_nodes {
            child_doc.nodes_mut().push(child);
        }
        *node.children_mut() = Some(child_doc);

        self.ser.current_node = Some(node);
        Ok(())
    }
}
