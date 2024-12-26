#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
#[macro_use]
extern crate std_plus;
mod authentication;

pub use authentication::state::State;

use serde::de::value;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Represents a claim used for authentication/authorization purposes.
///
/// The `Claim` struct encapsulates a type-value pair and related optional fields.
/// This can represent concepts such as tokens, user claims, or access permissions.
///
/// # Fields
///
/// - `_type`: The type of claim being represented (e.g., `role`, `user_id`, `email`).  
/// - `value`: The actual value of the claim (e.g., `user_id`, token, access key, or identifier).
/// - `issuer`:  The issuer of the claim, identifying the origin or authority of this claim.
/// - `subject`: The subject of the claim, representing context-specific information.  
/// - `properties`: Additional properties that might be associated with the claim.  
///
/// # Example
///
/// ```rust
/// use std::borrow::Cow;
/// use std::collections::HashMap;
/// use std_plus::string;
///
/// let claim = Claim::new("email", string!("auth-rs@example.com"))
/// ```
#[derive(Clone, PartialEq, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Claim {
    pub r#type: Cow<'static, str>,
    pub value: String,
    pub issuer: Option<String>,
    pub label: Option<Cow<'static, str>>,
    pub additional_claims: Option<Vec<Claim>>,
}

impl Claim {
    /// Creates a new `Claim` with an owned type and value.
    ///
    /// # Arguments
    /// - `r#type`: The type of the claim (as a `String`).
    /// - `value`: The value of the claim (as a `String`).
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new("type".to_string(), "value".to_string());
    /// ```
    pub fn new(r#type: String, value: String) -> Self {
        Self {
            r#type: Cow::Owned(r#type),
            value,
            issuer: None,
            label: None,
            additional_claims: None,
        }
    }

    /// Sets the `issuer` field of the claim.
    ///
    /// # Arguments
    /// - `issuer`: The issuer of the claim (as a `String`).
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new("type".to_string(), "value".to_string())
    ///     .with_issuer("issuer".to_string());
    /// ```
    pub fn with_issuer(mut self, issuer: String) -> Self {
        self.issuer = Some(issuer);
        self
    }

    /// Sets the `label` field of the claim.
    ///
    /// # Arguments
    /// - `label`: A label for the claim (as a `String`).
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new("type".to_string(), "value".to_string())
    ///     .with_label("label".to_string());
    /// ```
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(Cow::Owned(label));
        self
    }

    /// Sets the `additional_claims` field for the claim instance.
    ///
    /// This method allows you to associate a list of additional claims with the current claim.
    ///
    /// # Arguments
    /// - `additional_claims`: A vector of additional `Claim` instances to associate with this claim.
    ///
    /// # Returns
    /// Returns a new `Claim` instance with the specified additional claims set.
    ///
    /// # Example
    /// ```
    /// let additional_claims = vec![
    ///     Claim::new("type1".to_string(), "value1".to_string()),
    ///     Claim::new("type2".to_string(), "value2".to_string()),
    /// ];
    ///
    /// let claim = Claim::new("type".to_string(), "value".to_string())
    ///     .with_additional_claims(additional_claims);
    /// ```
    pub fn with_additional_claims(self, additional_claims: Vec<Claim>) -> Self {
        Self {
            additional_claims: Some(additional_claims),
            ..self
        }
    }

    /// Creates a new `Claim` with a borrowed type and value.
    ///
    /// # Arguments
    /// - `r#type`: The type of the claim (as a static string).
    /// - `value`: The value of the claim (as a `String`).
    ///
    /// # Example
    /// ```
    /// let claim = Claim::from("type", "value".to_string());
    /// ```
    pub fn from(r#type: &'static str, value: String) -> Self {
        Self {
            r#type: Cow::Borrowed(r#type),
            value,
            issuer: None,
            label: None,
            additional_claims: None,
        }
    }
}

trait ClaimBucket {
    type Key;

    /// Checks if the bucket is empty.
    fn is_empty(&self) -> bool;

    fn clear(&mut self);

    /// Inserts a single claim associated with a label.
    fn insert(&mut self, label: &Self::Key, claim: Claim);

    /// Inserts multiple claims associated with a label.
    fn insert_all<T: IntoIterator<Item = Claim>>(&mut self, label: &Self::Key, claims: T);

    /// Gets an immutable reference to all claims associated with a label.
    fn get(&self, label: &Self::Key) -> Option<&Vec<Claim>>;

    /// Gets a mutable reference to all claims associated with a label.
    fn get_mut(&mut self, label: &Self::Key) -> Option<&mut Vec<Claim>>;

    /// Gets claims that satisfy a predicate function.
    fn get_with<F: Fn(&Claim) -> bool>(&self, label: &Self::Key, predicate: F) -> Option<&Vec<Claim>>;

    /// Gets a mutable reference to claims that satisfy a predicate function.
    fn get_mut_with<F: Fn(&Claim) -> bool>(&mut self, label: &Self::Key, predicate: F) -> Option<&mut Vec<Claim>>;

    /// Deletes claims matching a predicate function from the bucket.
    /// Returns the deleted claims if found.
    fn delete_with<F: Fn(&Claim) -> bool>(&mut self, label: &Self::Key, predicate: F) -> Option<Vec<Claim>>;

    /// Deletes all claims associated with a label.
    /// Returns the deleted claims if the label existed.
    fn delete_all(&mut self, label: &Self::Key) -> Option<Vec<Claim>>;

    /// Updates claims associated with a label using a predicate and an update function.
    /// Returns the count of updated claims.
    fn update_with<F, U>(&mut self, label: &Self::Key, predicate: F, updater: U) -> usize
    where
        F: Fn(&Claim) -> bool,
        U: Fn(&mut Claim);
}
