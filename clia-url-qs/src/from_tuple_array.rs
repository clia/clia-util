use serde::Serialize;

/// Format an array of tuples to the URL query string.
/// 
/// ```rust
/// let params = [("foo", "bar"), ("baz", "quux")];
/// assert_eq!(clia_util::url::query_string::collection::tuple_array::format(&params), "foo=bar&baz=quux");
/// ```
/// 
/// If input is invalid, return error message.
pub fn from_tuple_array<T: Serialize + ?Sized>(arr: &T) -> String {
    match serde_urlencoded::to_string(arr) {
        Ok(body) => body,
        Err(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tuple_array() {
        let params = [("foo", "bar"), ("baz", "quux")];
        assert_eq!(from_tuple_array(&params), "foo=bar&baz=quux");
    }
}
