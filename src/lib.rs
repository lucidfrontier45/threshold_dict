#![allow(clippy::non_ascii_literal)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

pub struct ThresholdDict<K, V> {
    tree: BTreeMap<K, V>,
    default_value: Option<V>,
}

impl<K: Ord, V> From<BTreeMap<K, V>> for ThresholdDict<K, V> {
    fn from(tree: BTreeMap<K, V>) -> Self {
        Self {
            tree,
            default_value: None,
        }
    }
}

impl<K: Ord, V: Eq> PartialEq for ThresholdDict<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree && self.default_value == other.default_value
    }
}

impl<K: Ord, V: Eq> Eq for ThresholdDict<K, V> {}

impl<K: Ord, V> ThresholdDict<K, V> {
    /// default constructor
    pub fn new(kv: Vec<(K, V)>) -> Self {
        Self::with_default_value(kv, None)
    }

    /// constructor with default value
    pub fn with_default_value(kv: Vec<(K, V)>, default_value: Option<V>) -> Self {
        let tree = BTreeMap::from_iter(kv);
        Self {
            tree,
            default_value,
        }
    }

    /// set the default value
    pub fn set_default_value(&mut self, default_value: Option<V>) {
        self.default_value = default_value;
    }

    /// The query method.
    /// A value corresponding the smallest key which is greater than the query key is returned.
    pub fn query(&self, key: &K) -> Option<&V> {
        let query = (Excluded(key), Unbounded);
        let result = self.tree.range(query).next();
        result.map(|(_, v)| v).or_else(|| self.get_default_value())
    }

    fn get_default_value(&self) -> Option<&V> {
        self.default_value.as_ref()
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::ThresholdDict;

    #[test]
    fn test_from_btree_map() {
        let map = BTreeMap::from([(10, 100), (20, 150), (50, 300)]);
        let dict = ThresholdDict::from(map);
        let correct_dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)]);
        assert!(dict == correct_dict);
    }

    #[test]
    fn test_set_default_value() {
        let mut dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)]);
        dict.set_default_value(Some(500));
        let correct_dict =
            ThresholdDict::with_default_value(vec![(10, 100), (20, 150), (50, 300)], Some(500));
        assert!(dict == correct_dict);
    }

    #[test]
    fn test_query() {
        let dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)]);
        assert_eq!(dict.query(&0), Some(&100));
        assert_eq!(dict.query(&10), Some(&150));
        assert_eq!(dict.query(&15), Some(&150));
        assert_eq!(dict.query(&40), Some(&300));
        assert_eq!(dict.query(&50), None);
        assert_eq!(dict.query(&60), None);
    }

    #[test]
    fn test_query_with_default_value() {
        let dict =
            ThresholdDict::with_default_value(vec![(10, 100), (20, 150), (50, 300)], Some(500));
        assert_eq!(dict.query(&0), Some(&100));
        assert_eq!(dict.query(&10), Some(&150));
        assert_eq!(dict.query(&15), Some(&150));
        assert_eq!(dict.query(&40), Some(&300));
        assert_eq!(dict.query(&50), Some(&500));
        assert_eq!(dict.query(&60), Some(&500));
    }
}
