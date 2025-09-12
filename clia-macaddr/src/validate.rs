use mac_address::{MacAddress, MacParseError};
use std::str::FromStr;

/// Parse MAC address from string
pub fn parse_mac_addr(s: &str) -> Result<MacAddress, MacParseError> {
    MacAddress::from_str(s)
}

/// Check if a MAC address string is valid
pub fn is_valid_mac_addr(s: &str) -> bool {
    parse_mac_addr(s).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_mac_addr() {
        assert!(is_valid_mac_addr("aa:bb:cc:dd:ee:ff"));
        assert!(is_valid_mac_addr("AA:BB:CC:DD:EE:FF"));
        assert!(!is_valid_mac_addr("invalid"));
        assert!(!is_valid_mac_addr("aa:bb:cc:dd:ee"));
        assert!(!is_valid_mac_addr("aa:bb:cc:dd:ee:gg"));
    }
}
