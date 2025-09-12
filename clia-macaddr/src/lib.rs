mod range;
mod validate;

pub use range::count_addresses_between;
pub use range::count_addresses_between_str;
pub use range::get_addresses_between;
pub use range::get_addresses_between_str;
pub use range::get_addresses_between_str_as_strings;
pub use validate::is_valid_mac_addr;
pub use validate::parse_mac_addr;

use mac_address::MacAddress;

/// Format MAC address to standard format (aa:bb:cc:dd:ee:ff)
pub fn format_mac_addr(addr: &MacAddress) -> String {
    addr.to_string().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mac_addr() {
        let addr = parse_mac_addr("aa:bb:cc:dd:ee:ff").unwrap();
        assert_eq!(format_mac_addr(&addr), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn test_is_valid_mac_addr() {
        assert!(is_valid_mac_addr("aa:bb:cc:dd:ee:ff"));
        assert!(!is_valid_mac_addr("invalid"));
    }

    #[test]
    fn test_count_addresses_between() {
        let addr1 = parse_mac_addr("00:00:00:00:00:00").unwrap();
        let addr2 = parse_mac_addr("00:00:00:00:00:01").unwrap();
        assert_eq!(count_addresses_between(&addr1, &addr2), 2);

        // Test reverse order
        assert_eq!(count_addresses_between(&addr2, &addr1), 2);
    }

    #[test]
    fn test_larger_range() {
        let addr1 = parse_mac_addr("00:00:00:00:00:00").unwrap();
        let addr2 = parse_mac_addr("00:00:00:00:00:ff").unwrap();
        assert_eq!(count_addresses_between(&addr1, &addr2), 256);
    }

    #[test]
    fn test_count_addresses_between_str() {
        // Test with string parameters
        let count = count_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
        assert_eq!(count, 2);

        // Test reverse order
        let count = count_addresses_between_str("00:00:00:00:00:01", "00:00:00:00:00:00").unwrap();
        assert_eq!(count, 2);

        // Test larger range
        let count = count_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:ff").unwrap();
        assert_eq!(count, 256);

        // Test with invalid MAC address
        assert!(count_addresses_between_str("invalid", "00:00:00:00:00:01").is_err());
    }

    #[test]
    fn test_get_addresses_between() {
        let addr1 = parse_mac_addr("00:00:00:00:00:00").unwrap();
        let addr2 = parse_mac_addr("00:00:00:00:00:01").unwrap();
        let addresses = get_addresses_between(&addr1, &addr2);
        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].to_string().to_lowercase(), "00:00:00:00:00:00");
        assert_eq!(addresses[1].to_string().to_lowercase(), "00:00:00:00:00:01");
    }

    #[test]
    fn test_get_addresses_between_str() {
        let addresses =
            get_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].to_string().to_lowercase(), "00:00:00:00:00:00");
        assert_eq!(addresses[1].to_string().to_lowercase(), "00:00:00:00:00:01");

        // Test invalid
        assert!(get_addresses_between_str("invalid", "00:00:00:00:00:01").is_err());
    }

    #[test]
    fn test_get_addresses_between_str_as_strings() {
        let addresses =
            get_addresses_between_str_as_strings("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0], "00:00:00:00:00:00");
        assert_eq!(addresses[1], "00:00:00:00:00:01");

        // Test invalid
        assert!(get_addresses_between_str_as_strings("invalid", "00:00:00:00:00:01").is_err());
    }
}
