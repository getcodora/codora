use std::{borrow::Cow, collections::HashMap};

// TODO Implement other API and Study the Code well to fish out bugs or any issues!
#[derive(Clone)]
pub enum InnerEntry {
    Single(Cow<'static, str>),
    Multiple(Vec<Cow<'static, str>>),
}

impl FromIterator<Cow<'static, str>> for InnerEntry {
    fn from_iter<T: IntoIterator<Item = Cow<'static, str>>>(iter: T) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();
        match vec.len() {
            | 0 => InnerEntry::Single(Cow::Borrowed("")),
            | 1 => InnerEntry::Single(vec.into_iter().next().unwrap()),
            | _ => InnerEntry::Multiple(vec),
        }
    }
}

impl PartialEq for InnerEntry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            | (InnerEntry::Single(a), InnerEntry::Single(b)) => a == b,
            | (InnerEntry::Multiple(a), InnerEntry::Multiple(b)) => a == b,
            | (InnerEntry::Single(a), InnerEntry::Multiple(b)) => b.len() == 1 && b[0] == *a,
            | (InnerEntry::Multiple(a), InnerEntry::Single(b)) => a.len() == 1 && a[0] == *b,
        }
    }
}

impl std::ops::Index<usize> for InnerEntry {
    type Output = Cow<'static, str>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            | InnerEntry::Single(v) if index == 0 => v,
            | InnerEntry::Single(_) => panic!("Index out of bounds"),
            | InnerEntry::Multiple(v) => &v[index],
        }
    }
}

impl InnerEntry {
    fn iter(&self) -> Iter<'_> {
        match self {
            | InnerEntry::Single(v) => Iter::Single(Some(v)),
            | InnerEntry::Multiple(v) => Iter::Multiple(v.iter()),
        }
    }
}

pub enum Iter<'a> {
    Single(Option<&'a Cow<'static, str>>),
    Multiple(std::slice::Iter<'a, Cow<'static, str>>),
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            | Iter::Single(v) => v.take(),
            | Iter::Multiple(iter) => iter.next(),
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        match self {
            | Iter::Single(Some(_)) => 1,
            | Iter::Single(None) => 0,
            | Iter::Multiple(iter) => iter.len(),
        }
    }
}

impl Extend<Cow<'static, str>> for InnerEntry {
    fn extend<T: IntoIterator<Item = Cow<'static, str>>>(&mut self, iter: T) {
        match self {
            | InnerEntry::Single(v) => {
                let mut vec = Vec::new();
                vec.push(v.clone());
                vec.extend(iter);
                *self = InnerEntry::Multiple(vec);
            }
            | InnerEntry::Multiple(v) => v.extend(iter),
        }
    }
}

impl IntoIterator for InnerEntry {
    type Item = Cow<'static, str>;
    type IntoIter = IntoIterInner;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            | InnerEntry::Single(v) => IntoIterInner::Single(Some(v)),
            | InnerEntry::Multiple(v) => IntoIterInner::Multiple(v.into_iter()),
        }
    }
}

pub enum IntoIterInner {
    Single(Option<Cow<'static, str>>),
    Multiple(std::vec::IntoIter<Cow<'static, str>>),
}

impl Iterator for IntoIterInner {
    type Item = Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            | IntoIterInner::Single(v) => v.take(),
            | IntoIterInner::Multiple(iter) => iter.next(),
        }
    }
}

impl From<&'static str> for InnerEntry {
    fn from(value: &'static str) -> Self {
        InnerEntry::Single(Cow::Borrowed(value))
    }
}

impl From<String> for InnerEntry {
    fn from(value: String) -> Self {
        InnerEntry::Single(Cow::Owned(value))
    }
}

// Implement for Vec<T> where T can convert into Cow<'static, str>
impl<T> From<Vec<T>> for InnerEntry
where
    T: Into<Cow<'static, str>>,
{
    fn from(values: Vec<T>) -> Self {
        let values = values
            .into_iter()
            .map(Into::into)
            .collect();

        InnerEntry::Multiple(values)
    }
}
impl std::fmt::Debug for InnerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | InnerEntry::Single(v) => write!(f, "[{:?}]", v),
            | InnerEntry::Multiple(v) => f.debug_list().entries(v).finish(),
        }
    }
}

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
        V: Into<InnerEntry>,
    {
        let map = self
            .properties
            .get_or_insert_with(HashMap::new);

        match map.get_mut(key) {
            | Some(entry) => entry.extend(value.into()),
            | None => {
                map.insert(key, value.into());
            }
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
        state.add("add", vec!["hey"]);
        state.add("add", "plus".to_string());
        state.add("add", vec!["hey".to_string()]);

        println!("{:?}", state);
    }
}
