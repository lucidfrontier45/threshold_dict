#![allow(clippy::type_complexity)]

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Bound::{Excluded, Unbounded};

pub struct ThresholdDict<K, V> {
    tree: BTreeMap<K, V>,
}

impl<K: Ord, V> From<Vec<(K, V)>> for ThresholdDict<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let tree = BTreeMap::from_iter(kv);
        Self::new(tree)
    }
}

impl<K: Ord, V> ThresholdDict<K, V> {
    /// default constructor
    pub fn new(tree: BTreeMap<K, V>) -> Self {
        Self { tree }
    }

    /// The query method.
    /// A value corresponding the smallest key which is greater than the query key is returned.
    pub fn query(&self, key: &K) -> Option<&V> {
        let query = (Excluded(key), Unbounded);
        let result = self.tree.range(query).next();
        result.map(|(_, v)| v)
    }
}

impl<K: Ord + Clone, V: Clone> Clone for ThresholdDict<K, V> {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree.clone(),
        }
    }
}

impl<K: Ord + Debug, V: Debug> Debug for ThresholdDict<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThresholdDict")
            .field("tree", &self.tree)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::ThresholdDict;

    #[test]
    fn test_from_btree_map() {
        let map = BTreeMap::from([(10, 100), (20, 150), (50, 300)]);
        let dict = ThresholdDict::new(map);
        let correct_dict = ThresholdDict::from(vec![(10, 100), (20, 150), (50, 300)]);
        assert_eq!(dict.tree, correct_dict.tree);
    }

    #[test]
    fn test_clone() {
        let dict = ThresholdDict::from(vec![(10, 100), (20, 150), (50, 300)]);
        let dict2 = dict.clone();
        assert_eq!(dict.tree, dict2.tree);
    }

    #[test]
    fn test_query() {
        let dict = ThresholdDict::from(vec![(10, 100), (20, 150), (50, 300)]);
        assert_eq!(dict.query(&0), Some(&100));
        assert_eq!(dict.query(&10), Some(&150));
        assert_eq!(dict.query(&15), Some(&150));
        assert_eq!(dict.query(&40), Some(&300));
        assert_eq!(dict.query(&50), None);
        assert_eq!(dict.query(&60), None);
    }
}
