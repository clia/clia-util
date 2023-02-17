# clia-url-qs

Utility methods for URL query string.

## Usage

`from_tuple_array`

```rust
let params = [("foo", "bar"), ("baz", "quux")];
assert_eq!(clia_url_qs::from_tuple_array(&params).unwrap(), "foo=bar&baz=quux");
```
