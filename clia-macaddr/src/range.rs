use mac_address::MacAddress;

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

/// Get all MAC addresses between two MAC addresses (inclusive)
pub fn get_addresses_between(addr1: &MacAddress, addr2: &MacAddress) -> Vec<MacAddress> {
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

    let (start, end) = if val1 <= val2 {
        (val1, val2)
    } else {
        (val2, val1)
    };

    let mut addresses = Vec::new();
    for val in start..=end {
        let bytes = [
            ((val >> 40) & 0xFF) as u8,
            ((val >> 32) & 0xFF) as u8,
            ((val >> 24) & 0xFF) as u8,
            ((val >> 16) & 0xFF) as u8,
            ((val >> 8) & 0xFF) as u8,
            (val & 0xFF) as u8,
        ];
        addresses.push(MacAddress::new(bytes));
    }
    addresses
}

/// Get all MAC addresses between two MAC address strings (inclusive)
pub fn get_addresses_between_str(
    addr1: &str,
    addr2: &str,
) -> Result<Vec<MacAddress>, mac_address::MacParseError> {
    let mac1 = crate::validate::parse_mac_addr(addr1)?;
    let mac2 = crate::validate::parse_mac_addr(addr2)?;
    Ok(get_addresses_between(&mac1, &mac2))
}

/// Calculate the number of addresses between two MAC address strings (inclusive)
pub fn count_addresses_between_str(
    addr1: &str,
    addr2: &str,
) -> Result<u64, mac_address::MacParseError> {
    let mac1 = crate::validate::parse_mac_addr(addr1)?;
    let mac2 = crate::validate::parse_mac_addr(addr2)?;
    Ok(count_addresses_between(&mac1, &mac2))
}

/// Get all MAC addresses between two MAC address strings as strings (inclusive)
pub fn get_addresses_between_str_as_strings(
    addr1: &str,
    addr2: &str,
) -> Result<Vec<String>, mac_address::MacParseError> {
    let addresses = get_addresses_between_str(addr1, addr2)?;
    Ok(addresses
        .into_iter()
        .map(|addr| addr.to_string().to_lowercase())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::parse_mac_addr;

    #[test]
    fn test_count_addresses_between() {
        let addr1 = parse_mac_addr("00:00:00:00:00:00").unwrap();
        let addr2 = parse_mac_addr("00:00:00:00:00:01").unwrap();
        assert_eq!(count_addresses_between(&addr1, &addr2), 2);

        // Test reverse order
        assert_eq!(count_addresses_between(&addr2, &addr1), 2);
    }

    #[test]
    fn test_get_addresses_between() {
        let addr1 = parse_mac_addr("00:00:00:00:00:00").unwrap();
        let addr2 = parse_mac_addr("00:00:00:00:00:01").unwrap();
        let addresses = get_addresses_between(&addr1, &addr2);
        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].to_string().to_lowercase(), "00:00:00:00:00:00");
        assert_eq!(addresses[1].to_string().to_lowercase(), "00:00:00:00:00:01");

        // Test reverse order
        let addresses_rev = get_addresses_between(&addr2, &addr1);
        assert_eq!(addresses, addresses_rev);
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
