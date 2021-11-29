use std::net::{AddrParseError, Ipv4Addr};

pub fn ipv4_to_long(ip: &str) -> Result<i64, AddrParseError> {
    let ip_addr: Ipv4Addr = ip.parse()?;

    let ip_long = ip_addr
        .octets()
        .iter()
        .fold(0, |acc, octet| acc * 256 + *octet as i64);

    Ok(ip_long)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_to_long_valid_ip() {
        assert_eq!(ipv4_to_long("127.0.0.1").unwrap(), 2130706433);
    }

    #[test]
    fn test_ipv4_to_long_unvalid_ip() {
        assert!(ipv4_to_long("127.0.0").is_err(), "{}", true);
    }
}
