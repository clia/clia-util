# clia-macaddr

A Rust library for MAC address parsing, formatting, and calculation utilities.

## Features

- **Parse MAC addresses** from string format
- **Format MAC addresses** to standard lowercase format
- **Calculate the number of addresses** between two MAC addresses

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
clia-macaddr = "0.1.0"
```

### Examples

```rust
use clia_macaddr::{parse_mac_addr, format_mac_addr, count_addresses_between, count_addresses_between_str};

// Parse a MAC address from string
let addr1 = parse_mac_addr("aa:bb:cc:dd:ee:ff").unwrap();
let addr2 = parse_mac_addr("AA:BB:CC:DD:EE:FF").unwrap();

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

### `format_mac_addr(addr: &MacAddress) -> String`

Formats a MAC address to the standard lowercase colon-separated format: `aa:bb:cc:dd:ee:ff`

### `count_addresses_between(addr1: &MacAddress, addr2: &MacAddress) -> u64`

Calculates the number of MAC addresses between two addresses (inclusive). The order of addresses doesn't matter - the function automatically handles both directions.

### `count_addresses_between_str(addr1: &str, addr2: &str) -> Result<u64, MacParseError>`

A convenient version of `count_addresses_between` that accepts MAC address strings directly. Parses the strings internally and returns an error if either string is not a valid MAC address format. The order of addresses doesn't matter.

## Dependencies

This library uses the [`mac_address`](https://crates.io/crates/mac_address) crate for MAC address representation and parsing.

## License

This project is licensed under the same terms as your choice.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
