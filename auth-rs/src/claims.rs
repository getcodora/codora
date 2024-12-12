pub use claim::Claim;
pub use claim_map::ClaimMap;

mod claim {
    use super::ClaimMap;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
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
    /// let claim = Claim::new("email", string!("auth-rs@example.com"))
    /// ```
    #[derive(Clone, PartialEq, Debug, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Claim {
        _type: Cow<'static, str>,
        value: String,
        issuer: Option<String>,
        subject: Option<ClaimMap>,

        /// alias - properties
        /// Option used cause we don't wanna carry empty hashmap all around
        props: Option<HashMap<String, String>>,
    }

    impl Claim {
        pub fn from(_type: &'static str, value: String) -> Self {
            Self {
                _type: Cow::Borrowed(_type),
                value,
                issuer: None,
                props: None,
                subject: None,
            }
        }

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
        /// let claim = Claim::new(...).with_subject(ClaimMap::new(...));
        /// ```
        pub fn with_subject(self, subject: ClaimMap) -> Self {
            Self {
                subject: Some(subject),
                ..self
            }
        }

        // TODO -> Work on the properties api
        // /// alias add_properties
        // pub fn add_props(&mut self, key: &str, value: String) -> &mut Self {
        //     self.get_props()
        //         .insert(key.into(), value);
        //     self
        // }

        // pub fn update_props(&mut self, key: &str, value: String) ->  {
        //     self.props.map(|prop| )
        // }

        // pub fn

        // fn get_props(&mut self) -> &mut HashMap<String, String> {
        //     self.props.get_or_insert(HashMap::new())
        // }
    }
}
mod claim_map {
    use super::Claim;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use std::{borrow::Cow, collections::HashMap};

    // Add extend and other useful iterator

    #[derive(Clone, PartialEq, Debug, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct ClaimMap {
        // So we don't carry empty map all around when not used
        claim_map: Option<HashMap<String, Vec<Claim>>>,
    }

    impl ClaimMap {
        pub fn new() -> Self {
            Self { claim_map: None }
        }

        pub fn add_claims(&mut self, label: &str, claims: Vec<Claim>) -> &mut Self {
            self.populate_claims((label.into(), claims));
            self
        }

        fn populate_claims(&mut self, (label, claims): (String, Vec<Claim>)) {
            let map = self
                .claim_map
                .get_or_insert(HashMap::new());

            map.insert(label, claims);
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::{claim::Claim, ClaimMap};

    #[test]
    fn test_claim() -> anyhow::Result<()> {
        Ok(())
    }
}
