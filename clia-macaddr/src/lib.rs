use mac_address::{MacAddress, MacParseError};
use std::str::FromStr;

/// Parse MAC address from string
pub fn parse_mac_addr(s: &str) -> Result<MacAddress, MacParseError> {
    MacAddress::from_str(s)
}

/// Format MAC address to standard format (aa:bb:cc:dd:ee:ff)
pub fn format_mac_addr(addr: &MacAddress) -> String {
    addr.to_string().to_lowercase()
}

/// Calculate the number of addresses between two MAC addresses (inclusive)
pub fn count_addresses_between(addr1: &MacAddress, addr2: &MacAddress) -> u64 {
    let bytes1 = addr1.bytes();
    let val1 = ((bytes1[0] as u64) << 40)
        | ((bytes1[1] as u64) << 32)
        | ((bytes1[2] as u64) << 24)
        | ((bytes1[3] as u64) << 16)
        | ((bytes1[4] as u64) << 8)
        | (bytes1[5] as u64);
    let bytes2 = addr2.bytes();
    let val2 = ((bytes2[0] as u64) << 40)
        | ((bytes2[1] as u64) << 32)
        | ((bytes2[2] as u64) << 24)
        | ((bytes2[3] as u64) << 16)
        | ((bytes2[4] as u64) << 8)
        | (bytes2[5] as u64);
    if val1 <= val2 {
        val2 - val1 + 1
    } else {
        val1 - val2 + 1
    }
}

/// Calculate the number of addresses between two MAC address strings (inclusive)
pub fn count_addresses_between_str(addr1: &str, addr2: &str) -> Result<u64, MacParseError> {
    let mac1 = parse_mac_addr(addr1)?;
    let mac2 = parse_mac_addr(addr2)?;
    Ok(count_addresses_between(&mac1, &mac2))
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
}
