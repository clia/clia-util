use anyhow::Result;
use serde::Serialize;

/// Format an array of tuples to the URL query string.
///
/// ```rust
/// let params = [("foo", "bar"), ("baz", "quux")];
/// assert_eq!(clia_url_qs::from_tuple_array(&params).unwrap(), "foo=bar&baz=quux");
/// ```
///
/// If input is invalid, return error message.
pub fn from_tuple_array<T: Serialize + ?Sized>(arr: &T) -> Result<String> {
    serde_urlencoded::to_string(arr).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tuple_array() {
        let params = [("foo", "bar"), ("baz", "quux")];
        assert_eq!(from_tuple_array(&params).unwrap(), "foo=bar&baz=quux");
    }
}
