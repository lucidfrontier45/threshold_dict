pub struct ThresholdDict<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    default_value: V,
    linear_search_max_len: usize,
}

const DEFAULT_LINEAR_SEARCH_MAX_LEN: usize = 10;

impl<K: PartialOrd, V> ThresholdDict<K, V> {
    pub fn new(mut kv: Vec<(K, V)>, default_value: V) -> Self {
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

    pub fn with_linear_search_max_len(
        kv: Vec<(K, V)>,
        default_value: V,
        linear_search_max_len: usize,
    ) -> Self {
        let mut dict = Self::new(kv, default_value);
        dict.linear_search_max_len = linear_search_max_len;
        dict
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.keys.is_empty() {
            return None;
        }

        if self.keys.len() < self.linear_search_max_len {
            self.linear_search(key)
        } else {
            self.binary_search(key)
        }
    }

    fn linear_search(&self, key: &K) -> Option<&V> {
        let n = self.keys.len();
        for i in 0..n {
            if key < &self.keys[i] {
                return self.values.get(i);
            }
        }
        Some(&self.default_value)
    }

    fn binary_search(&self, key: &K) -> Option<&V> {
        let i = self.keys.partition_point(|x| x <= key);
        if i == self.keys.len() {
            return Some(&self.default_value);
        }
        self.values.get(i)
    }
}

#[cfg(test)]
mod test {
    use super::ThresholdDict;

    #[test]
    fn test_step_dict_linear() {
        let dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)], 500);

        assert_eq!(dict.linear_search(&0).unwrap(), &100);
        assert_eq!(dict.linear_search(&10).unwrap(), &150);
        assert_eq!(dict.linear_search(&15).unwrap(), &150);
        assert_eq!(dict.linear_search(&50).unwrap(), &500);
        assert_eq!(dict.linear_search(&60).unwrap(), &500);
    }

    #[test]
    fn test_step_dict_binary() {
        let dict = ThresholdDict::new(vec![(10, 100), (20, 150), (50, 300)], 500);

        assert_eq!(dict.binary_search(&0).unwrap(), &100);
        assert_eq!(dict.binary_search(&10).unwrap(), &150);
        assert_eq!(dict.binary_search(&15).unwrap(), &150);
        assert_eq!(dict.binary_search(&50).unwrap(), &500);
        assert_eq!(dict.binary_search(&60).unwrap(), &500);
    }
}
