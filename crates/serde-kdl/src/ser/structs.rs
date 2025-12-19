use std::mem;

use super::Serializer;
use crate::{
    error::{Error, Result},
    ser::SerializeFieldValue,
};
use kdl::{KdlDocument, KdlNode};
use serde::ser::{SerializeStruct, SerializeStructVariant};

// Struct serializer
pub struct SerializeStructImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) is_root: bool,
}

impl SerializeStruct for SerializeStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Serialize the field value using a temporary serializer
        let mut field_serializer = Serializer::new_for_field();
        value.serialize(&mut field_serializer)?;
        let mut field_doc = field_serializer.into_document();

        // Get the first node from the serialized field value
        let Some(mut field_node) = field_doc.nodes_mut().pop() else {
            // If the field serialized to nothing, skip it
            return Ok(());
        };

        field_node.set_name(key);

        // Add the field node to the appropriate location
        if self.is_root {
            // Root level: add field node directly to document
            self.ser.document.nodes_mut().push(field_node);
        } else {
            // Nested struct: add field node to the current context's children
            if let Some((_name, children)) = self.ser.node_stack.last_mut() {
                children.push(field_node);
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if self.is_root {
            // Root level: nothing to do, fields already added to document
            Ok(())
        } else {
            // Nested: finalize the node context
            self.ser.pop_node_context()
        }
    }
}

// Struct variant serializer
pub struct SerializeStructVariantImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) variant: String,
}

impl SerializeStructVariant for SerializeStructVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Serialize the field value using a temporary serializer
        let field_doc = value.into_option_node()?;

        // Get the first node from the serialized field value
        let Some(mut node) = field_doc else {
            // If the field serialized to nothing, skip it
            return Ok(());
        };
        node.set_name(key);

        // Add to the document directly (will be wrapped with variant type annotation at end)
        self.ser.document.nodes_mut().push(node);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Create a node with "-" as name and the variant as type annotation
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        node.set_ty(self.variant);

        // Move all the field nodes we added to the document into this node's children
        let mut child_doc = KdlDocument::new();
        mem::swap(self.ser.document.nodes_mut(), child_doc.nodes_mut());
        *node.children_mut() = Some(child_doc);
        self.ser.current_node = Some(node);

        Ok(())
    }
}
