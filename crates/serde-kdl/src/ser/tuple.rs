use crate::error::{Error, Result};
use kdl::{KdlEntry, KdlNode, KdlValue};
use serde::ser::{SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use super::Serializer;

// Tuple serializer
pub struct SerializeTupleImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) items: Vec<KdlValue>,
}

impl SerializeTuple for SerializeTupleImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        for item in self.items {
            node.entries_mut().push(KdlEntry::new(item));
        }
        self.ser.current_node = Some(node);
        Ok(())
    }
}

// Tuple struct serializer
pub struct SerializeTupleStructImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) items: Vec<KdlValue>,
}

impl SerializeTupleStruct for SerializeTupleStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Add the tuple values as entries to the current node
        if let Some((_name, children)) = self.ser.node_stack.last_mut() {
            let mut node = KdlNode::new("tuple");
            for item in self.items {
                node.entries_mut().push(KdlEntry::new(item));
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
    pub(crate) items: Vec<KdlValue>,
}

impl SerializeTupleVariant for SerializeTupleVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Create a node with the variant as type annotation
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        node.set_ty(kdl::KdlIdentifier::from(self.variant.as_str()));

        // Add all tuple items as arguments
        for item in self.items {
            node.entries_mut().push(KdlEntry::new(item));
        }

        self.ser.current_node = Some(node);
        Ok(())
    }
}
