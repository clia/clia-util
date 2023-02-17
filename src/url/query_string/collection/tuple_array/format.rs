use serde::Serialize;

/// Format an array of tuples to the URL query string.
/// 
/// ```rust
/// let params = [("foo", "bar"), ("baz", "quux")];
/// assert_eq!(clia_util::url::query_string::collection::tuple_array::format(&params), "foo=bar&baz=quux");
/// ```
/// 
/// If input is invalid, return error message.
pub fn format<T: Serialize + ?Sized>(arr: &T) -> String {
    match serde_urlencoded::to_string(arr) {
        Ok(body) => body,
        Err(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let params = [("foo", "bar"), ("baz", "quux")];
        assert_eq!(format(&params), "foo=bar&baz=quux");
    }
}
