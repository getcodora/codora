use std::{borrow::Cow, collections::HashMap};

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
/// let claim = Claim::from("email", string!("auth-rs@example.com"))
/// ```
#[derive(new, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Claim {
    _type: Cow<'static, str>,
    value: String,
    issuer: Option<String>,
    subject: Option<ClaimInner>,

    /// alias - properties
    pub props: Option<HashMap<String, String>>,
}

#[derive(new, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ClaimInner {
    label: Cow<'static, str>,
    claim: Vec<Claim>,
}

#[derive(new, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Claims {
    claims: Vec<ClaimInner>,
}

impl Claim {
    /// Sets the `issuer` for the claim.
    ///
    /// # Arguments
    /// * `issuer` - A string slice representing the issuer.
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new(...).with_issuer("IssuerName");
    /// ```
    pub fn with_issuer(self, issuer: &str) -> Self {
        Self {
            issuer: Some(issuer.into()),
            ..self
        }
    }

    /// Sets the `subject` for the claim.
    ///
    /// # Arguments
    /// * `subject` - A `Claims` struct representing the subject.
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new(...).with_subject(ClaimInner::new(...));
    /// ```
    pub fn with_subject(self, subject: ClaimInner) -> Self {
        Self {
            subject: Some(subject),
            ..self
        }
    }

    /// Sets additional `properties` for the claim.
    ///
    /// # Arguments
    /// * `props` - A map of key-value string pairs representing custom properties.
    ///
    /// # Example
    /// ```
    /// let claim = Claim::new(...).with_props(HashMap::new());
    /// ```
    pub fn with_props(self, props: HashMap<String, String>) -> Self {
        Self {
            props: Some(props),
            ..self
        }
    }
}
/// Implements conversion from a tuple into a `Claim`.
///
/// This allows for easy creation of a `Claim` using a `(&str, String)` tuple.
///
/// # Arguments
/// * `_type`: A static string slice representing the claim type.
/// * `value`: A dynamically created string representing the claim value.
///
/// # Example
/// ```rust
/// let claim = Claim::from("email", String::from("auth-rs@example.com"));
/// ```
impl From<(&'static str, String)> for Claim {
    fn from((_type, value): (&'static str, String)) -> Self {
        Self {
            _type: Cow::Borrowed(_type),
            value,
            issuer: None,
            props: None,
            subject: None,
        }
    }
}
