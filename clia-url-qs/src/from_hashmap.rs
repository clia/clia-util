use anyhow::Result;
use serde::Serialize;

/// Format an array of tuples to the URL query string.
///
/// ```rust
/// let mut params = HashMap::new();
/// params.insert("foo", "bar");
/// params.insert("baz", "quux");
///
/// let res = from_hashmap(&params).unwrap();
/// ```
///
/// If input is invalid, return error message.
pub fn from_hashmap<T: Serialize + ?Sized>(arr: &T) -> Result<String> {
    serde_urlencoded::to_string(arr).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_from_hashmap() {
        let mut params = HashMap::new();
        params.insert("foo", "bar");
        params.insert("baz", "quux");

        let res = from_hashmap(&params).unwrap();
        print!("{res}");
        // assert_eq!(from_hashmap(&params).unwrap(), "foo=bar&baz=quux");
    }
}
