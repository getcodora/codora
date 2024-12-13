mod claim {
    use super::claims::Claims;
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
}
mod claims {
    use super::claim::Claim;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use std::{borrow::Cow, collections::HashMap};

    // Add extend and other useful iterator

    #[derive(Clone, PartialEq, Debug, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Claims {
        claims: Option<HashMap<String, Vec<Claim>>>,
    }

    impl Claims {
        /// Creates a new, empty `Claims` instance.
        pub fn new() -> Self {
            Self { claims: None }
        }

        /// Checks if the `Claims` instance is empty.
        pub fn is_empty(&self) -> bool {
            self.claims
                .as_ref()
                .map_or(true, |claims| claims.is_empty())
        }

        /// Inserts a single claim under the specified label.
        pub fn insert(&mut self, label: &str, claim: Claim) -> &mut Self {
            let claims = self
                .claims
                .get_or_insert_with(HashMap::new);

            claims
                .entry(label.to_string())
                .or_default()
                .push(claim);
            self
        }

        /// Inserts multiple claims under the specified label.
        pub fn insert_all<T>(&mut self, label: &str, claims: T) -> &mut Self
        where
            T: IntoIterator<Item = Claim>,
        {
            let existing_claims = self
                .claims
                .get_or_insert_with(HashMap::new);

            existing_claims
                .entry(label.to_string())
                .or_default()
                .extend(claims);
            self
        }

        /// Retrieves all claims for a specific label.
        pub fn get_all(&self, label: &str) -> Option<Vec<Claim>> {
            self.claims
                .as_ref()
                .and_then(|claims| claims.get(label).cloned())
        }

        /// Retrieves all claims for a specific label that satisfy a given predicate.
        pub fn get_all_with<F>(&self, label: &str, predicate: F) -> Option<Vec<Claim>>
        where
            F: Fn(&Claim) -> bool,
        {
            self.claims.as_ref().and_then(|claims| {
                claims.get(label).map(|claims| {
                    claims
                        .iter()
                        .filter(|claim| predicate(claim))
                        .cloned()
                        .collect()
                })
            })
        }

        /// Removes all claims for a specific label.
        pub fn remove_all(&mut self, label: &str) -> Option<Vec<Claim>> {
            self.claims
                .as_mut()
                .and_then(|claims| claims.remove(label))
        }

        /// Removes all claims for a specific label that satisfy a given predicate.
        pub fn remove_all_with<F>(&mut self, label: &str, predicate: F) -> Option<Vec<&Claim>>
        where
            F: Fn(&Claim) -> bool,
        {
            // self.claims.as_mut().and_then(|claims| {
            //     claims.get_mut(label).map(|claims| {
            //         let (to_remove, to_keep): (Vec<_>, Vec<_>) = claims
            //             .iter()
            //             .partition(|claim| predicate(claim));
            //         *claims = to_keep;
            //         to_remove
            //     })
            // })

            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::{claim::Claim, claims::Claims};

    #[test]
    fn test_claim() -> anyhow::Result<()> {
        let mut claims = Claims::new();

        let claim1 = Claim::from("email", "auth-rs@example.com".to_string());
        let claim2 = Claim::from("name", "auth-rs".to_string());

        claims.insert("google", claim1.clone());
        claims.insert("google", claim2.clone());

        // Retrieve all claims
        if let Some(all_claims) = claims.get_all("google") {
            println!("{:#?}", all_claims);
        }

        // Retrieve claims with a predicate
        if let Some(filtered_claims) = claims.get_all_with("google", |claim| claim.r#type == "email") {
            println!("{:#?}", filtered_claims);
        }

        // Remove all claims for a label
        claims.remove_all("google");
        Ok(())
    }
}
