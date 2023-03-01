#![allow(clippy::type_complexity)]

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

pub struct ThresholdDict<K, V> {
    tree: BTreeMap<K, V>,
    default_func: Box<dyn Fn(&K) -> Option<V>>,
}

impl<K: Ord, V: Copy> From<Vec<(K, V)>> for ThresholdDict<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let tree = BTreeMap::from_iter(kv);
        Self::new(tree)
    }
}

impl<K: Ord, V: Copy> ThresholdDict<K, V> {
    /// default constructor
    pub fn new(tree: BTreeMap<K, V>) -> Self {
        let default_func = Box::new(|_: &K| None::<V>);
        Self::with_default_func(tree, default_func)
    }

    /// constructor with default value
    pub fn with_default_func(
        tree: BTreeMap<K, V>,
        default_func: Box<dyn Fn(&K) -> Option<V>>,
    ) -> Self {
        Self { tree, default_func }
    }

    /// set the default value
    pub fn set_default_func(&mut self, default_func: Box<dyn Fn(&K) -> Option<V>>) {
        self.default_func = default_func;
    }

    /// The query method.
    /// A value corresponding the smallest key which is greater than the query key is returned.
    pub fn query(&self, key: &K) -> Option<V> {
        let query = (Excluded(key), Unbounded);
        let result = self.tree.range(query).next();
        result.map(|(_, v)| *v).or_else(|| (self.default_func)(key))
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
    fn test_set_default_value() {
        let mut dict = ThresholdDict::from(vec![(10, 100), (20, 150), (50, 300)]);
        let default_func = Box::new(|_: &u32| Some(500));
        dict.set_default_func(default_func.clone());
        assert_eq!((dict.default_func)(&60), default_func(&60));
    }

    #[test]
    fn test_query() {
        let dict = ThresholdDict::from(vec![(10, 100), (20, 150), (50, 300)]);
        assert_eq!(dict.query(&0), Some(100));
        assert_eq!(dict.query(&10), Some(150));
        assert_eq!(dict.query(&15), Some(150));
        assert_eq!(dict.query(&40), Some(300));
        assert_eq!(dict.query(&50), None);
        assert_eq!(dict.query(&60), None);
    }

    #[test]
    fn test_query_with_default_value() {
        let default_value = 500;
        let default_func = Box::new(move |_: &u32| Some(default_value));
        let tree = BTreeMap::from([(10, 100), (20, 150), (50, 300)]);
        let dict = ThresholdDict::with_default_func(tree, default_func);
        assert_eq!(dict.query(&0), Some(100));
        assert_eq!(dict.query(&10), Some(150));
        assert_eq!(dict.query(&15), Some(150));
        assert_eq!(dict.query(&40), Some(300));
        assert_eq!(dict.query(&50), Some(500));
        assert_eq!(dict.query(&60), Some(500));
    }
}
