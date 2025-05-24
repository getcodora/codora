use codora_util::impl_error__and_display;
use serde::Serialize;
use serde_json::{Value, to_value};
use std::{borrow::Cow, collections::HashMap};

pub trait IntoError {
    type Output;

    fn into_error(self) -> Self::Output;
}

pub trait Validate<T = ()>
where
    Self::Error: IntoError,
{
    type Error;

    fn validate(&self, context: &T) -> Result<(), Self::Error>;
}

// Impl validate for Rust types from array to vec to btreemap to hashmap

#[derive(Debug, Clone, Default)]
pub struct ValidationError {
    pub error: Option<Cow<'static, str>>,
    attributes: Option<HashMap<Cow<'static, str>, Value>>,
}

impl<T> From<T> for ValidationError
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        ValidationError::new(value)
    }
}

impl ValidationError {
    #[rustfmt::skip]
    pub fn new<T>(error: T) -> Self where T: Into<Cow<'static, str>>  {
        let error = error.into();
        Self { error: Some(error), attributes: None }
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

impl_error__and_display!(ValidationError);

impl From<ValidationError> for Option<Cow<'static, str>> {
    fn from(value: ValidationError) -> Self {
        value.error
    }
}
