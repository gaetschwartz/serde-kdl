//! Document parsing
//!
//! This module handles parsing of complete KDL documents.

use crate::parse::node::KdlNode;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub struct KdlDocument {
    pub nodes: Vec<KdlNode>,
}

impl KdlDocument {
    #[must_use]
    pub fn nodes(&self) -> &[KdlNode] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<KdlNode> {
        &mut self.nodes
    }

    #[must_use]
    pub fn from_nodes(nodes: Vec<KdlNode>) -> Self {
        KdlDocument { nodes }
    }
}

impl Parse for KdlDocument {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            let node = input.parse::<KdlNode>()?;
            nodes.push(node);
        }

        Ok(KdlDocument { nodes })
    }
}

impl PartialEq<kdl::KdlDocument> for KdlDocument {
    fn eq(&self, other: &kdl::KdlDocument) -> bool {
        if self.nodes.len() != other.nodes().len() {
            return false;
        }

        self.nodes == other.nodes()
    }
}
