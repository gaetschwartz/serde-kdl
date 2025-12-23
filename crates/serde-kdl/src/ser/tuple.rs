use super::Serializer;
use crate::{
    DEFAULT_NODE_NAME,
    error::{Error, Result},
    ser::SerializeFieldValue,
};
use kdl::{KdlDocument, KdlEntry, KdlNode};
use serde::ser::{SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

/// Check if a node is "simple" - can be represented as an inline entry
fn is_simple_node(node: &KdlNode) -> bool {
    node.entries().len() == 1 && node.children().is_none() && node.ty().is_none()
}

// Tuple serializer
pub struct SerializeTupleImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) child_nodes: Vec<KdlNode>,
}

impl SerializeTuple for SerializeTupleImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut node = value.into_node()?;
        node.set_name(DEFAULT_NODE_NAME);
        self.child_nodes.push(node);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(DEFAULT_NODE_NAME);

        // Check if all elements are simple (can be inline)
        let all_simple = self.child_nodes.iter().all(is_simple_node);

        if all_simple {
            // Extract values as inline entries
            for child in self.child_nodes {
                if let Some(entry) = child.entries().first() {
                    node.entries_mut()
                        .push(KdlEntry::new(entry.value().clone()));
                }
            }
        } else {
            // Use children block
            let mut child_doc = KdlDocument::new();
            child_doc.nodes_mut().extend(self.child_nodes);
            *node.children_mut() = Some(child_doc);
        }

        self.ser.current_node = Some(node);
        Ok(())
    }
}

// Tuple struct serializer
pub struct SerializeTupleStructImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) child_nodes: Vec<KdlNode>,
}

impl SerializeTupleStruct for SerializeTupleStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut node = value.into_node()?;
        node.set_name(DEFAULT_NODE_NAME);
        self.child_nodes.push(node);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Check if all elements are simple (can be inline)
        let all_simple = self.child_nodes.iter().all(is_simple_node);

        if let Some((_name, children)) = self.ser.node_stack.last_mut() {
            let mut node = KdlNode::new(DEFAULT_NODE_NAME);

            if all_simple {
                // Extract values as inline entries
                for child in self.child_nodes {
                    if let Some(entry) = child.entries().first() {
                        node.entries_mut()
                            .push(KdlEntry::new(entry.value().clone()));
                    }
                }
            } else {
                // Use children block
                let mut child_doc = KdlDocument::new();
                child_doc.nodes_mut().extend(self.child_nodes);
                *node.children_mut() = Some(child_doc);
            }

            children.push(node);
        }
        self.ser.pop_node_context()
    }
}

// Tuple variant serializer
pub struct SerializeTupleVariantImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) variant: String,
    pub(crate) child_nodes: Vec<KdlNode>,
}

impl SerializeTupleVariant for SerializeTupleVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut node = value.into_node()?;
        node.set_name(DEFAULT_NODE_NAME);
        self.child_nodes.push(node);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Create a node with the variant as type annotation
        let mut node = KdlNode::new(DEFAULT_NODE_NAME);
        node.set_ty(kdl::KdlIdentifier::from(self.variant.as_str()));

        // Check if all elements are simple (can be inline)
        let all_simple = self.child_nodes.iter().all(is_simple_node);

        if all_simple {
            // Extract values as inline entries (arguments)
            for child in self.child_nodes {
                if let Some(entry) = child.entries().first() {
                    node.entries_mut()
                        .push(KdlEntry::new(entry.value().clone()));
                }
            }
        } else {
            // Use children block
            let mut child_doc = KdlDocument::new();
            child_doc.nodes_mut().extend(self.child_nodes);
            *node.children_mut() = Some(child_doc);
        }

        self.ser.current_node = Some(node);
        Ok(())
    }
}
