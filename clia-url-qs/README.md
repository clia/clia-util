# clia-url-qs

Utility methods for URL query string.

Member of [clia-util](https://github.com/clia/clia-util).

## Usage

`from_tuple_array`

```rust
let params = [("foo", "bar"), ("baz", "quux")];
assert_eq!(clia_url_qs::from_tuple_array(&params).unwrap(), "foo=bar&baz=quux");
```

`from_hashmap`

```rust
let mut params = HashMap::new();
params.insert("foo", "bar");
params.insert("baz", "quux");

let res = clia_url_qs::from_hashmap(&params).unwrap();
```