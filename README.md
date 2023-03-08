# threshold-dict
A data structure to find smallest key that is larger than the query.

Suppose we have a following weight dependent price table.

```
weight,price
100,10
200,15
500,30
otherwise,50
```

The price is decided by the smallest entry whose `weight` key is greater than the query. 

*examples*

- weight=90 -> price=10
- weight=100 -> price=15
- weight=250 -> price=30
- weight=600 -> price=50

## Install

```sh
cargo add threshold_dict
```

## Usage

A `ThresholdDict` can be created by passing kv pairs and a default func. If query is greater than or equal to all of the keys, the default func is used.

```rust
let kv_pairs = vec![(100, 10), (200, 15), (500, 30)];
let tree: BTreeMap<u32, u32> = kv_pairs.into_iter().collect();
let dict = ThresholdDict::new(tree);

assert_eq!(dict.query(&90), Some(&10));
assert_eq!(dict.query(600), None);
```