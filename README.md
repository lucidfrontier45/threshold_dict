# threshold_dict
A data structure to find smallest key that is larger than the query.

Suppose we have a following weight dependent price table.

```
weight,price
100,10
200,15
500,30
,50
```

The price is decided by the smallest entry whose `weight` key is larger than the query. 

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

A `ThresholdDict` can be created by passing kv pairs and a default value. If query is larger than any of the keys, the default value is used.

```rust
let kv_pairs = vec![(100, 10), (200, 15), (500, 30)];
let default_value = 50;
let dict = ThresholdDict::new(kv_paris, default_value);

assert_eq!(dict.get(90), 10);
```