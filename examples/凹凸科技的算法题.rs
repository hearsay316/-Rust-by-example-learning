use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum IpConversionError {
    InvalidFormat,
    InvalidCharacter,
    NumberOutOfRange,
}

fn ipv4_to_u32(ip: &str) -> Result<u32, IpConversionError> {
    let mut parts = Vec::new();
    let mut current_number = String::new();
    let mut has_space_in_number = false;

    for c in ip.chars() {
        match c {
            '0'..='9' => {
                if has_space_in_number {
                    return Err(IpConversionError::InvalidCharacter);
                }
                current_number.push(c);
            }
            '.' => {
                if current_number.is_empty() {
                    return Err(IpConversionError::InvalidFormat);
                }
                let num = current_number
                    .parse::<u8>()
                    .map_err(|_| IpConversionError::NumberOutOfRange)?;
                parts.push(num);
                current_number.clear();
                has_space_in_number = false;
            }
            ' ' => {
                if !current_number.is_empty() {
                    has_space_in_number = true;
                }
            }
            _ => return Err(IpConversionError::InvalidCharacter),
        }
    }

    if !current_number.is_empty() {
        let num = current_number
            .parse::<u8>()
            .map_err(|_| IpConversionError::NumberOutOfRange)?;
        parts.push(num);
    }

    if parts.len() != 4 {
        return Err(IpConversionError::InvalidFormat);
    }

    let result = ((parts[0] as u32) << 24)
        | ((parts[1] as u32) << 16)
        | ((parts[2] as u32) << 8)
        | (parts[3] as u32);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert_eq!(ipv4_to_u32("172.168.5.1"), Ok(2896692481));
        assert_eq!(ipv4_to_u32("0.0.0.0"), Ok(0));
        assert_eq!(ipv4_to_u32("255.255.255.255"), Ok(4294967295));
    }

    #[test]
    fn test_invalid_format() {
        assert_eq!(
            ipv4_to_u32("172.168.5"),
            Err(IpConversionError::InvalidFormat)
        );
        assert_eq!(
            ipv4_to_u32("172.168.5.1.1"),
            Err(IpConversionError::InvalidFormat)
        );
    }

    #[test]
    fn test_invalid_character() {
        assert_eq!(
            ipv4_to_u32("172.168.5.a"),
            Err(IpConversionError::InvalidCharacter)
        );
        assert_eq!(
            ipv4_to_u32("172.168.5.-1"),
            Err(IpConversionError::InvalidCharacter)
        );
    }

    #[test]
    fn test_number_out_of_range() {
        assert_eq!(
            ipv4_to_u32("172.168.5.256"),
            Err(IpConversionError::NumberOutOfRange)
        );
        assert_eq!(
            ipv4_to_u32("172.168.5.1000"),
            Err(IpConversionError::NumberOutOfRange)
        );
    }

    #[test]
    fn test_space_in_number() {
        assert_eq!(
            ipv4_to_u32("1 72.168.5.1"),
            Err(IpConversionError::InvalidCharacter)
        );
        assert_eq!(ipv4_to_u32("172 .168.5.1"), Ok(2896692481));
    }
}

fn main() {
    let ip = "172. 168.5.1";
    match ipv4_to_u32(ip) {
        Ok(result) => println!("{} => {}", ip, result),
        Err(e) => println!("Error: {:?}", e),
    }
}
