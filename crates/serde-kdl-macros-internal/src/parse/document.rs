//! Document parsing
//!
//! This module handles parsing of complete KDL documents.

use crate::parse::{
    comments::{MaybeSlashed, SlashDash},
    node::KdlNode,
};
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub struct KdlDocument {
    pub nodes: Vec<MaybeSlashed<KdlNode>>,
}

impl KdlDocument {
    pub fn nodes(&self) -> impl Iterator<Item = &KdlNode> {
        self.nodes.iter().filter_map(|n| match n {
            MaybeSlashed::Item(node) => Some(node),
            MaybeSlashed::Slashed(_, _) => None,
        })
    }

    pub fn ignored_nodes(&self) -> impl Iterator<Item = (&SlashDash, &KdlNode)> {
        self.nodes.iter().filter_map(|n| match n {
            MaybeSlashed::Slashed(sd, node) => Some((sd, node)),
            MaybeSlashed::Item(_) => None,
        })
    }

    #[must_use]
    pub fn from_nodes(nodes: impl IntoIterator<Item = MaybeSlashed<KdlNode>>) -> Self {
        KdlDocument {
            nodes: nodes.into_iter().collect(),
        }
    }
}

impl Parse for KdlDocument {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse::<MaybeSlashed<KdlNode>>()?);
        }

        Ok(KdlDocument { nodes })
    }
}

impl PartialEq<kdl::KdlDocument> for KdlDocument {
    fn eq(&self, other: &kdl::KdlDocument) -> bool {
        if self.nodes.len() != other.nodes().len() {
            return false;
        }

        std::iter::Iterator::eq(self.nodes(), other.nodes())
    }
}
