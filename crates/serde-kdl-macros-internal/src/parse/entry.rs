use std::borrow::Cow;

use syn::Token;

use crate::parse::{identifier::KdlIdentifier, type_annotation::MaybeAnnotated, value::KdlValue};

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone, PartialEq)]
pub struct KdlEntry {
    pub name: Option<KdlIdentifier>, // According to Section 3.7, property keys must be String values
    pub value: MaybeAnnotated<KdlValue>,
}

impl KdlEntry {
    pub fn new_prop(name: impl Into<KdlIdentifier>, value: impl Into<KdlValue>) -> KdlEntry {
        KdlEntry {
            name: Some(name.into()),
            value: MaybeAnnotated::new(value.into()),
        }
    }

    pub fn new(value: impl Into<KdlValue>) -> KdlEntry {
        KdlEntry {
            name: None,
            value: MaybeAnnotated::new(value.into()),
        }
    }

    pub fn new_typed_prop(
        name: impl Into<KdlIdentifier>,
        value: MaybeAnnotated<impl Into<KdlValue>>,
    ) -> KdlEntry {
        KdlEntry {
            name: Some(name.into()),
            value: MaybeAnnotated {
                item: value.item.into(),
                type_annotation: value.type_annotation,
            },
        }
    }

    pub fn new_typed_arg(value: MaybeAnnotated<impl Into<KdlValue>>) -> KdlEntry {
        KdlEntry {
            name: None,
            value: MaybeAnnotated {
                item: value.item.into(),
                type_annotation: value.type_annotation,
            },
        }
    }

    pub fn set_ty(&mut self, type_annotation: KdlIdentifier) {
        self.value.type_annotation = Some(type_annotation);
    }

    #[must_use]
    pub fn ty(&self) -> Option<&KdlIdentifier> {
        self.value.type_annotation.as_ref()
    }

    #[must_use]
    pub fn name(&self) -> Option<&KdlIdentifier> {
        self.name.as_ref()
    }

    #[must_use]
    pub fn name_str(&self) -> Option<Cow<'_, str>> {
        self.name.as_ref().map(|n| n.value())
    }
}

impl syn::parse::Parse for KdlEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Try to parse as argument (literal value or identifier)
        let value = input.parse::<MaybeAnnotated<KdlValue>>()?;

        let value_line = value.span().start().line;

        // Check if it's a property (key=value) or argument (value)
        if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            // eprintln!("[node({name})] Found '=' token ");
            // It's a property
            let key = value.try_into()?;
            let prop_value = input.parse::<MaybeAnnotated<KdlValue>>()?;
            let prop_line = prop_value.span().start().line;
            if value_line != prop_line {
                return Err(syn::Error::new(
                    prop_value.span(),
                    "Property values must be on the same line as their keys.",
                ));
            }

            Ok(KdlEntry {
                name: Some(key),
                value: prop_value,
            })
        } else {
            // It's an argument
            Ok(KdlEntry { name: None, value })
        }
    }
}

impl PartialEq<kdl::KdlEntry> for KdlEntry {
    fn eq(&self, other: &kdl::KdlEntry) -> bool {
        match (&self.name, other.name()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }
        if &self.value.item != other.value() {
            return false;
        }
        match (&self.value.type_annotation, other.ty()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }
        true
    }
}
