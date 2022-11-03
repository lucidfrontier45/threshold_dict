#![allow(clippy::non_ascii_literal)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

pub struct ThresholdDict<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    default_value: Option<V>,
    linear_search_max_len: usize,
}

const DEFAULT_LINEAR_SEARCH_MAX_LEN: usize = 10;

impl<K: PartialOrd, V> ThresholdDict<K, V> {
    /// default constructor
    pub fn new(mut kv: Vec<(K, V)>, default_value: Option<V>) -> Self {
        kv.sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());
        let mut keys = vec![];
        let mut values = vec![];
        for (k, v) in kv {
            keys.push(k);
            values.push(v);
        }
        Self {
            keys,
            values,
            default_value,
            linear_search_max_len: DEFAULT_LINEAR_SEARCH_MAX_LEN,
        }
    }

    /// constructor with custom `linear_search_max_len` parameter
    pub fn with_linear_search_max_len(
        kv: Vec<(K, V)>,
        default_value: Option<V>,
        linear_search_max_len: usize,
    ) -> Self {
        let mut dict = Self::new(kv, default_value);
        dict.linear_search_max_len = linear_search_max_len;
        dict
    }

    /// The query method.
    /// A value corresponding the minimum key which is larger than the query key is returned.
    /// If the internal key-value list is empty, the default value is always returned.
    /// The search algorithm is selected from linear search or binary search governed by internal `linear_search_max_len` parameter.
    pub fn query(&self, key: &K) -> Option<&V> {
        if self.keys.is_empty() {
            return self.get_default_value();
        }

        if self.keys.len() < self.linear_search_max_len {
            self.linear_search(key)
        } else {
            self.binary_search(key)
        }
    }

    fn get_default_value(&self) -> Option<&V> {
        self.default_value.as_ref()
    }

    fn linear_search(&self, key: &K) -> Option<&V> {
        let n = self.keys.len();
        for i in 0..n {
            if key <= &self.keys[i] {
                return self.values.get(i);
            }
        }
        self.get_default_value()
    }

    fn binary_search(&self, key: &K) -> Option<&V> {
        let i = self.keys.partition_point(|x| x < key);
        if i == self.keys.len() {
            return self.get_default_value();
        }
        self.values.get(i)
    }
}

#[cfg(test)]
mod test {
    use super::ThresholdDict;

    #[test]
    fn test_linear() {
        let dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)], Some(500));

        assert_eq!(dict.linear_search(&0), Some(&100));
        assert_eq!(dict.linear_search(&10), Some(&100));
        assert_eq!(dict.linear_search(&15), Some(&150));
        assert_eq!(dict.linear_search(&50), Some(&300));
        assert_eq!(dict.linear_search(&60), Some(&500));
    }

    #[test]
    fn test_binary() {
        let dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)], Some(500));

        assert_eq!(dict.binary_search(&0), Some(&100));
        assert_eq!(dict.binary_search(&10), Some(&100));
        assert_eq!(dict.binary_search(&15), Some(&150));
        assert_eq!(dict.binary_search(&50), Some(&300));
        assert_eq!(dict.binary_search(&60), Some(&500));
    }

    #[test]
    fn test_nonnull_default_value() {
        let dict = ThresholdDict::new(vec![], Some(500));
        assert_eq!(dict.query(&0), Some(&500));
        assert_eq!(dict.query(&10), Some(&500));
        assert_eq!(dict.query(&15), Some(&500));
        assert_eq!(dict.query(&50), Some(&500));
        assert_eq!(dict.query(&60), Some(&500));
    }

    #[test]
    fn test_null_default_value() {
        let dict: ThresholdDict<i32, i32> = ThresholdDict::new(vec![], None);
        assert_eq!(dict.query(&0), None);
        assert_eq!(dict.query(&10), None);
        assert_eq!(dict.query(&15), None);
        assert_eq!(dict.query(&50), None);
        assert_eq!(dict.query(&60), None);
    }
}
