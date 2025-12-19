use crate::error::{Error, Result};
use kdl::KdlValue;
use serde::de::Visitor;
use serde::{forward_to_deserialize_any, Deserializer as DeserializerTrait};

/// A deserializer that works directly with a single `KdlEntry`
/// This is useful for deserializing sequence elements that could be enums
pub(crate) struct EntryDeserializer<'de> {
    entry: &'de kdl::KdlEntry,
}

impl<'de> EntryDeserializer<'de> {
    pub(crate) fn new(entry: &'de kdl::KdlEntry) -> Self {
        Self { entry }
    }

    fn entry(&self) -> &kdl::KdlEntry {
        self.entry
    }
}

impl<'de> EntryDeserializer<'de> {
    fn deserialize_from_value<V>(&self, value: &KdlValue, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match value {
            KdlValue::String(s) => visitor.visit_str(s),
            KdlValue::Integer(i) => {
                // serde visitors for most numeric types only accept i64, not i128
                if let Ok(v) = i64::try_from(*i) {
                    visitor.visit_i64(v)
                } else {
                    visitor.visit_i128(*i)
                }
            }
            KdlValue::Float(f) => visitor.visit_f64(*f),
            KdlValue::Bool(b) => visitor.visit_bool(*b),
            KdlValue::Null => visitor.visit_unit(),
        }
    }
}

impl<'de> DeserializerTrait<'de> for EntryDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_from_value(self.entry().value(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // For simple values like strings in sequences, treat the value as the variant name
        if let KdlValue::String(s) = self.entry().value() {
            use serde::de::value::StrDeserializer;
            visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()))
        } else {
            // For other values, deserialize normally
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Fallback to normal any deserialization
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.entry().value() {
            KdlValue::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        unit unit_struct newtype_struct tuple
        tuple_struct map struct identifier ignored_any
    }
}
