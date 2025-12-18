//! Document parsing
//!
//! This module handles parsing of complete KDL documents.

use crate::ast::{KdlDocument, KdlNode};
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

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
