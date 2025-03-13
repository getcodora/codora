use crate::Validator;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
use thestack::new;

pub struct ValidatorContext<T, V> {
    validator: V,

    after_validator: Option<Box<dyn Fn(&T) -> Result<(), Error>>>,
    before_validator: Option<Box<dyn Fn(&T) -> Result<(), Error>>>,
}

impl<T, V> ValidatorContext<T, V>
where
    V: Validator<T>,
{
    pub fn validate(&self, payload: &T) -> Result<(), Error> {
        if let Some(ref h) = self.before_validator {
            h(payload)?;
        }

        self.validator.validate(payload)?;

        self.after_validator
            .as_ref()
            .map_or(Ok(()), |h| h(payload))
    }

    // If any of the handler return error we return the error
    pub fn after_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&T) -> Result<(), Error> + 'static,
    {
        self.after_validator
            .replace(Box::new(handler));
        self
    }

    pub fn before_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&T) -> Result<(), Error> + 'static,
    {
        self.before_validator
            .replace(Box::new(handler));
        self
    }
}

#[derive(Debug, new, Serialize)]
pub struct Error {
    reason: Cow<'static, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<InnerError>,
}

#[derive(Debug, Serialize)]
pub enum InnerError {
    Single(Cow<'static, str>),
    Multiple(HashMap<Cow<'static, str>, InnerError>),
}
