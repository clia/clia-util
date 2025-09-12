# clia-macaddr

A Rust library for MAC address parsing, formatting, and calculation utilities.

## Features

- **Parse MAC addresses** from string format
- **Validate MAC addresses** for correctness
- **Format MAC addresses** to standard lowercase format
- **Calculate the number of addresses** between two MAC addresses
- **Generate all MAC addresses** in a range

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
clia-macaddr = "0.2.0"
```

### Examples

```rust
use clia_macaddr::{
    parse_mac_addr, format_mac_addr, is_valid_mac_addr,
    count_addresses_between, count_addresses_between_str,
    get_addresses_between, get_addresses_between_str, get_addresses_between_str_as_strings
};

// Parse a MAC address from string
let addr1 = parse_mac_addr("aa:bb:cc:dd:ee:ff").unwrap();
let addr2 = parse_mac_addr("AA:BB:CC:DD:EE:FF").unwrap();

// Validate MAC address strings
println!("Is valid: {}", is_valid_mac_addr("aa:bb:cc:dd:ee:ff")); // true
println!("Is valid: {}", is_valid_mac_addr("invalid")); // false

// Format MAC address to lowercase
println!("{}", format_mac_addr(&addr1)); // "aa:bb:cc:dd:ee:ff"

// Calculate number of addresses between two MAC addresses
let start = parse_mac_addr("00:00:00:00:00:00").unwrap();
let end = parse_mac_addr("00:00:00:00:00:01").unwrap();
let count = count_addresses_between(&start, &end);
println!("Addresses between: {}", count); // 2 (inclusive)

// Calculate using string parameters directly (more convenient)
let count = count_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
println!("Addresses between: {}", count); // 2 (inclusive)

// Get all MAC addresses in a range (as MacAddress objects)
let addresses = get_addresses_between(&start, &end);
println!("Addresses: {:?}", addresses.iter().map(|a| a.to_string().to_lowercase()).collect::<Vec<_>>());

// Get all MAC addresses in a range using strings (returns MacAddress objects)
let addresses = get_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
println!("Addresses: {:?}", addresses.iter().map(|a| a.to_string().to_lowercase()).collect::<Vec<_>>());

// Get all MAC addresses in a range as strings (most convenient)
let address_strings = get_addresses_between_str_as_strings("00:00:00:00:00:00", "00:00:00:00:00:01").unwrap();
println!("Address strings: {:?}", address_strings); // ["00:00:00:00:00:00", "00:00:00:00:00:01"]

// Works with larger ranges
let count = count_addresses_between_str("00:00:00:00:00:00", "00:00:00:00:00:ff").unwrap();
println!("Addresses in range: {}", count); // 256

// Error handling for invalid MAC addresses
match count_addresses_between_str("invalid", "00:00:00:00:00:01") {
    Ok(count) => println!("Count: {}", count),
    Err(e) => println!("Parse error: {}", e),
}
```

## API Reference

### `parse_mac_addr(s: &str) -> Result<MacAddress, MacParseError>`

Parses a MAC address from a string. Supports various formats including:
- `aa:bb:cc:dd:ee:ff`
- `AA:BB:CC:DD:EE:FF`
- And other formats supported by the `mac_address` crate

### `is_valid_mac_addr(s: &str) -> bool`

Checks if a string represents a valid MAC address. Returns `true` if the string can be successfully parsed as a MAC address, `false` otherwise.

### `format_mac_addr(addr: &MacAddress) -> String`

Formats a MAC address to the standard lowercase colon-separated format: `aa:bb:cc:dd:ee:ff`

### `count_addresses_between(addr1: &MacAddress, addr2: &MacAddress) -> u64`

Calculates the number of MAC addresses between two addresses (inclusive). The order of addresses doesn't matter - the function automatically handles both directions.

### `count_addresses_between_str(addr1: &str, addr2: &str) -> Result<u64, MacParseError>`

A convenient version of `count_addresses_between` that accepts MAC address strings directly. Parses the strings internally and returns an error if either string is not a valid MAC address format. The order of addresses doesn't matter.

### `get_addresses_between(addr1: &MacAddress, addr2: &MacAddress) -> Vec<MacAddress>`

Returns all MAC addresses between two addresses (inclusive) as a vector of `MacAddress` objects. The order of input addresses doesn't matter - the function automatically determines the correct range.

### `get_addresses_between_str(addr1: &str, addr2: &str) -> Result<Vec<MacAddress>, MacParseError>`

A convenient version of `get_addresses_between` that accepts MAC address strings directly. Returns all MAC addresses in the range as `MacAddress` objects. Parses the strings internally and returns an error if either string is not a valid MAC address format.

### `get_addresses_between_str_as_strings(addr1: &str, addr2: &str) -> Result<Vec<String>, MacParseError>`

Returns all MAC addresses between two address strings as a vector of formatted strings. This is the most convenient function for getting address ranges when you need string output. All addresses are formatted in lowercase colon-separated format. Parses the strings internally and returns an error if either string is not a valid MAC address format.

## Dependencies

This library uses the [`mac_address`](https://crates.io/crates/mac_address) crate for MAC address representation and parsing.

## License

This project is licensed under the same terms as your choice.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
