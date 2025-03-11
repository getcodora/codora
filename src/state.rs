use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

// TODO
/*
 * Index
 * FromIterator
 * IntoIterator
 * PartialEQ EQ
 * Extend
 * Iterator for Iter
 * ExactIterator
*/
pub(super) enum InnerEntry {
    Single(Cow<'static, str>),
    Multiple(Vec<Cow<'static, str>>),
}

impl std::fmt::Debug for InnerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | Self::Single(arg0) => f
                .debug_tuple("Single")
                .field(arg0)
                .finish(),
            | Self::Multiple(arg0) => f
                .debug_tuple("Multiple")
                .field(arg0)
                .finish(),
        }
    }
}

// impl IntoIterator for InnerEntry {
//     type Item = &'static str;

//     type IntoIter = ();

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

/// # State
/// Used for holding authentication state during request lifecycle
/// ### Notes
/// * State contain's None to avoid carrying empty Map all around
#[derive(Default, Debug)]
pub struct State {
    properties: Option<HashMap<&'static str, InnerEntry>>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    /// Add a properties into the state
    ///
    /// # Example
    ///
    /// ```
    /// use weekend::security::State;
    /// let mut state = State::new();
    /// state.add("authenticated", "true");
    /// state.add("members", String::from("weekend"));
    /// state.add("name", "west");
    /// assert_eq!(state.len(), 3)
    /// ```
    pub fn add<V>(&mut self, key: &'static str, value: V)
    where
        V: Into<Cow<'static, str>>,
    {
        let entry = self
            .properties
            .get_or_insert_with(HashMap::new)
            .entry(key);

        match entry {
            #[rustfmt::skip]
            | Entry::Vacant(e) => { e.insert(InnerEntry::Single(value.into())); }
            | Entry::Occupied(mut e) => match e.get_mut() {
                | InnerEntry::Multiple(v) => v.push(value.into()),
                | InnerEntry::Single(v) => *e.get_mut() = InnerEntry::Multiple(vec![v.clone(), value.into()]),
            },
        }
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use weekend::security::State;
    ///
    /// let mut state = State::new();
    /// state.add("authenticated", "true");
    /// assert_eq!(state.contains_key("authenticated"), true);
    /// assert_eq!(state.contains_key("authenticated"), false);
    /// ```
    pub fn contains_key(&self, k: &'static str) -> bool {
        self.properties
            .as_ref()
            .map_or(false, |properties| properties.contains_key(k))
    }

    /// Check whether the state is empty or not.
    ///
    /// # Example
    ///
    /// ```
    /// use weekend::security::State;
    /// let mut state = State::new();
    /// assert!(state.is_empty());
    /// state.add("authenticated", "true");
    /// assert!(!state.is_empty())
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.properties
            .as_ref()
            .map_or(true, |properties| properties.is_empty())
    }

    /// Get the len of the state.
    ///
    /// # Example
    ///
    /// ```
    /// use weekend::security::State;
    /// let mut state = State::new();
    /// assert_eq!(state.len(), 0);
    /// state.add("authenticated", "true");
    /// assert_eq!(state.len(), 1)
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.properties
            .as_ref()
            .map_or(0, |properties| properties.len())
    }

    /// Clear the `State`.
    ///
    /// # Example
    ///
    /// ```
    /// # use weekend::security::State;
    /// let mut state = State::new();
    /// state.add("authenticated", "true");
    /// state.clear();
    /// assert!(state.is_empty())
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut properties) = self.properties {
            properties.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use super::State;

    #[test]
    fn test() {
        let mut state = State::new();
        state.add("add", "+");
        state.add("add", "plus");

        // assert_eq!(state.iter().next(), Some("+"));
        // assert_eq!(state.iter().next(), Some("plus"));
    }
}
