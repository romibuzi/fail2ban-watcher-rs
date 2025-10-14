use std::net::{AddrParseError, Ipv4Addr};

pub fn ipv4_to_u32(ip: &str) -> Result<u32, AddrParseError> {
    let ip_addr: Ipv4Addr = ip.parse()?;

    Ok(u32::from(ip_addr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_to_u32_valid_ip() {
        assert_eq!(ipv4_to_u32("127.0.0.1").unwrap(), 2130706433);
        assert_eq!(ipv4_to_u32("192.168.1.1").unwrap(), 3232235777);
        assert_eq!(ipv4_to_u32("0.0.0.0").unwrap(), 0);
        assert_eq!(ipv4_to_u32("255.255.255.255").unwrap(), 4294967295);
    }

    #[test]
    fn test_ipv4_to_u32_invalid_ip() {
        assert!(ipv4_to_u32("127.0.0").is_err(), "{}", true);
        assert!(ipv4_to_u32("").is_err());
    }
}
