use std::{borrow::Cow, collections::HashMap};

// TODO Implement other API and Study the Code well to fish out bugs or any issues!
// Implement IterMut and remove function iter_all, entry and other iterator trait for state

// A claim is a claim that can compose claim
/*

    |
    |--|

*/
#[derive(Default)]
pub struct Claim {
    value: Entry,
    visibility: Visibility,
    label: Cow<'static, str>,
}

#[derive(Default)]
pub enum Visibility {
    Private,

    #[default]
    Public,
}

#[derive(Default)]
pub enum Entry {
    #[default]
    Single,
}

/// # State
/// Used for holding authentication state during request lifecycle
/// ### Notes
/// * State contain's None to avoid carrying empty Map all around
#[derive(Default, Debug)]
pub struct State {
    properties: Option<HashMap<&'static str, Vec<Cow<'static, str>>>>,
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
    /// use codora::security::State;
    /// let mut state = State::new();
    /// state.add("authenticated", "true");
    /// state.add("members", String::from("codora"));
    /// state.add("name", "west");
    /// assert_eq!(state.len(), 3)
    /// ```
    pub fn add<V>(&mut self, key: &'static str, value: V)
    where
        V: IntoIterator<Item = Cow<'static, str>>,
    {
        // let map = self
        //     .properties
        //     .get_or_insert_with(HashMap::new);

        // match map.get_mut(key) {
        //     Some(entry) => entry.extend(value.into()),
        //     None => {
        //         map.insert(key, value.into());
        //     }
        // }

        todo!()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use codora::security::State;
    /// use codora::security::state::Entry;
    ///
    /// let mut state = State::new();
    /// state.add("role", "user");
    ///
    /// if let Some(entry) = state.get_mut("role") {
    ///     match entry {
    ///         Entry::Single(v) => {
    ///             *v = "admin".into();
    ///             assert_eq!(v, "admin");
    ///             assert_ne!(v, "user");
    ///         },
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub fn get_mut(&mut self, key: &'static str) -> Option<&mut Vec<Cow<'static, str>>> {
        // self.properties
        //     .as_mut()
        //     .and_then(|props| props.get_mut(key))

        todo!()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use codora::security::State;
    /// use codora::security::state::Entry;
    ///
    /// let mut state = State::new();
    /// state.add("role", "user");
    ///
    /// if let Some(entry) = state.get_mut("role") {
    ///     match entry {
    ///         Entry::Single(v) => {
    ///             assert_eq!(v, "user");
    ///         },
    ///         _ => {}
    ///     }
    /// }
    /// ```
    // pub fn get(&self, key: &'static str) -> Option<&Entry> {
    //     self.properties
    //         .as_ref()
    //         .and_then(|props| props.get(key))
    // }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use codora::security::State;
    ///
    /// let mut state = State::new();
    /// state.add("authenticated", "true");
    /// assert_eq!(state.contains_key("authenticated"), true);
    /// ```
    // pub fn contains_key(&self, k: &'static str) -> bool {
    //     self.properties
    //         .as_ref()
    //         .map_or(false, |properties| properties.contains_key(k))
    // }

    /// Check whether the state is empty or not.
    ///
    /// # Example
    ///
    /// ```
    /// use codora::security::State;
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
    /// use codora::security::State;
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
    /// # use codora::security::State;
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

/// Implements the [`Index`] trait to allow indexing into a [`State`] using string literals.
///
/// # Examples
///
/// ```
/// use codora::security::State;
/// use codora::security::state::Entry;
///
/// let mut state = State::new();
/// state.add("key", "value");
///
/// // Access value using index notation
/// // assert_eq!(&state["key"], &Entry::Single("new_value".into()));
/// ```
///
/// # Panics
///
/// Panics if the key does not exist in the state.
/// Consider using [`State::get`] if you want to handle missing keys gracefully.
// impl Index<&'static str> for State {
//     type Output = Entry;

//     fn index(&self, index: &'static str) -> &Self::Output {
//         self.get(index).unwrap()
//     }
// }

/// Implements the [`IndexMut`] trait to allow mutable indexing into a [`State`] using string literals.
///
/// # Examples
///
/// ```
/// use codora::security::State;
/// use codora::security::state::Entry;
///
/// let mut state = State::new();
/// state.add("key", "value");
///
/// // Modify value using mutable index notation
/// state["key"] = Entry::Single("new_value".into());
/// // assert_eq!(&state["key"], &Entry::Single("new_value".into()))
/// ```
///
/// # Panics
///
/// Panics if the key does not exist in the state.
/// Consider using [`State::get_mut`] if you want to handle missing keys gracefully.
// impl IndexMut<&'static str> for State {
//     #[inline]
//     fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
//         // Using unwrap is intentional here as index operations are meant to panic on missing keys
//         self.get_mut(index)
//             .expect("no entry found for key")
//     }
// }

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::State;

    #[test]
    fn test() {
        // let value = String::new();
        // let mut state = State::new();

        // state.add("add", "+");
        // state.add("add", "plus");
        // state.add("add", vec!["hey"]);
        // state.add("add", "plus".to_string());
        // state.add("add", vec!["hey".to_string()]);

        // let test_index = &mut state["add"];

        // println!("{:?}", test_index);
        // println!("{:?}", state);
    }
}
