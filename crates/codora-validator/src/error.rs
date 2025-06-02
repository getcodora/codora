use codora_util::impl_error__and_display;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Value, to_value};
use std::{borrow::Cow, collections::HashMap};

pub trait Validate<T = ()> {
    type Error;

    fn validate(&self, context: &T) -> Result<(), Self::Error>;
}

// Impl validate for Rust types from array to vec to btreemap to hashmap

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationError {
    pub error: Option<Cow<'static, str>>,
    attributes: Option<HashMap<Cow<'static, str>, Value>>,
}

impl<T> From<T> for ValidationError
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl ValidationError {
    pub fn new<T>(error: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        let error = error.into();
        Self {
            error: Some(error),
            attributes: None,
        }
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

    pub fn is_empty(&self) -> bool {
        self.error.is_none() && self.attributes.is_none()
    }

    pub fn skip(&self) -> bool {
        self.error.is_none()
    }
}

impl_error__and_display!(ValidationError);

impl Serialize for ValidationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.error {
            Some(ref error_message) => serializer.serialize_str(&error_message),
            None => serializer.serialize_unit(),
        }
    }
}

impl<'de> Deserialize<'de> for ValidationError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Cow<'static, str> = Cow::deserialize(deserializer)?;
        Ok(ValidationError::new(s))
    }
}

pub fn serialize_vec_error<S>(value: &Vec<ValidationError>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let filtered: Vec<_> = value
        .iter()
        .filter(|error| !error.skip())
        .collect();

    filtered.serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::{ValidationError, serialize_vec_error};
    use codora_util::new;
    use serde::Serialize;

    #[test]
    fn test_serialize_error() -> anyhow::Result<()> {
        #[derive(Debug, new, Serialize)]
        pub struct ErrorOfT {
            #[serde(skip_serializing_if = "ValidationError::skip")]
            foo: ValidationError,
            #[serde(skip_serializing_if = "ValidationError::skip")]
            bar: ValidationError,
            #[serde(skip_serializing_if = "ValidationError::skip")]
            bee: ValidationError,
            #[serde(serialize_with = "serialize_vec_error")]
            baz: Vec<ValidationError>,
        }

        let error = ValidationError::new("error");
        let json_string = serde_json::to_string(&error)?;

        assert_eq!(json_string, "\"error\"");

        let v_error = serde_json::from_str::<ValidationError>(&json_string)?;
        assert_eq!(v_error, error);

        let error_of_t = ErrorOfT::new(error.clone(), error.clone(), ValidationError::default(), vec![error, ValidationError::default()]);
        println!("{:#?}", error_of_t);

        let json_string = serde_json::to_string(&error_of_t)?;

        println!("{:#?}", json_string);
        Ok(())
    }
}
