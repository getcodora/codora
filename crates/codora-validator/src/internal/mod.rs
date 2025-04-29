use std::{borrow::Cow, collections::HashMap};

use codora_util::impl_error__and_display;
use serde::Serialize;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serializer};
use serde_json::{Value, to_value};

use std::fmt;

pub trait Validate<T = ()> {
    type Error;

    fn validate(&self, context: &T) -> Result<(), Self::Error>;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct Error {
    pub message: Option<Cow<'static, str>>,
    attributes: Option<HashMap<Cow<'static, str>, Value>>,
}

impl Error {
    #[rustfmt::skip]
    pub fn new<T>(message: T) -> Self where T: Into<Cow<'static, str>>  {
        let message = message.into();
        Self { message: Some(message), attributes: None }
    }

    pub fn add_params<K, T>(&mut self, name: K, value: &T)
    where
        K: Into<Cow<'static, str>>,
        T: Serialize,
    {
        let name = name.into();
        let attributes = self
            .attributes
            .get_or_insert_with(HashMap::new);
        attributes.insert(name, to_value(value).unwrap());
    }
}

impl_error__and_display!(Error);

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.message {
            Some(msg) => serializer.serialize_str(msg),
            None => serializer.serialize_str("v"),
        }
    }
}
struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = Error;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing an error message")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Error {
            message: Some(Cow::Owned(v.to_string())),
            attributes: None,
        })
    }
}
impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ErrorVisitor)
    }
}
