use serde::{
    de::{self, Visitor},
    Deserialize,
};
use std::fmt;

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("value must be true/false")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "true" => Ok(true),
            "false" => Ok(false),
            e => Err(E::custom(format!(
                "string {} cannot be converted to boolean",
                e
            ))),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(v)
    }
}

pub fn parse_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor)
}

#[derive(Deserialize)]
struct Wrap(#[serde(deserialize_with = "parse_bool")] bool);

pub fn parse_bool_opt<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(Option::<Wrap>::deserialize(deserializer)?.map(|v| v.0))
}
