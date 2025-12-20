//! Document parsing
//!
//! This module handles parsing of complete KDL documents.

use crate::parse::{comments::MaybeSlashed, node::KdlNode};
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub struct KdlDocument {
    pub nodes: Vec<KdlNode>,
    /// Nodes that are commented out with `/-`
    pub ignored_nodes: Vec<KdlNode>,
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
    pub fn ignored_nodes(&self) -> &[KdlNode] {
        &self.ignored_nodes
    }

    #[must_use]
    pub fn from_nodes(nodes: Vec<KdlNode>) -> Self {
        KdlDocument {
            nodes,
            ignored_nodes: Vec::new(),
        }
    }
}

impl Parse for KdlDocument {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();
        let mut ignored_nodes = Vec::new();

        while !input.is_empty() {
            match input.parse::<MaybeSlashed<KdlNode>>()? {
                MaybeSlashed::Item(n) => nodes.push(n),
                MaybeSlashed::Slashed(n) => ignored_nodes.push(n),
            }
        }

        Ok(KdlDocument {
            nodes,
            ignored_nodes,
        })
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
