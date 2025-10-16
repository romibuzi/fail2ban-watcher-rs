use csv::Reader;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
struct IPRange {
    start: u32,
    end: u32,
    country_code: String,
    country_name: String,
}

pub struct IP2Location {
    ranges: Vec<IPRange>,
}

impl IP2Location {
    pub fn from_reader<R>(reader: &mut Reader<R>) -> IP2Location
    where
        R: std::io::Read,
    {
        let ranges = reader
            .deserialize()
            .filter_map(|row| {
                let range: Option<IPRange> = row.unwrap_or(None);
                range
            })
            .collect();

        IP2Location { ranges }
    }

    pub fn find_country_name_of_ip(&self, target_ip: u32) -> Option<String> {
        match self.ranges.binary_search_by(|range| {
            if target_ip < range.start {
                Ordering::Greater
            } else if target_ip > range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(index) => Some(self.ranges[index].country_name.clone()),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;

    use super::*;

    #[test]
    fn test_build_with_valid_ip2location_data() {
        // Given
        let data = "3758094336,3758095359,HK,Hong Kong\n\
                          3758095360,3758095871,CN,China";
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data.as_bytes());

        // When
        let result = IP2Location::from_reader(&mut reader);

        // Then
        assert_eq!(
            result.ranges,
            vec![
                IPRange {
                    start: 3758094336,
                    end: 3758095359,
                    country_code: String::from("HK"),
                    country_name: String::from("Hong Kong"),
                },
                IPRange {
                    start: 3758095360,
                    end: 3758095871,
                    country_code: String::from("CN"),
                    country_name: String::from("China"),
                },
            ]
        );
    }

    #[test]
    fn test_build_with_invalid_ip2location_data() {
        // Given
        let data = "3758094336,3758095359,HK,Hong Kong\n\
                          3758095360,China";
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data.as_bytes());

        // When
        let result = IP2Location::from_reader(&mut reader);

        // Then
        assert_eq!(
            result.ranges,
            vec![IPRange {
                start: 3758094336,
                end: 3758095359,
                country_code: String::from("HK"),
                country_name: String::from("Hong Kong"),
            }]
        );
    }

    #[test]
    fn test_find_country_name_of_ip_found() {
        // Given
        let ip_range_one = IPRange {
            start: 1,
            end: 16777215,
            country_code: String::from("HK"),
            country_name: String::from("Hong Kong"),
        };
        let ip_range_two = IPRange {
            start: 16777216,
            end: 16785407,
            country_code: String::from("CN"),
            country_name: String::from("China"),
        };
        let ip2location = IP2Location {
            ranges: vec![ip_range_one.clone(), ip_range_two],
        };
        let target_ip = 100;

        // When
        let result = ip2location.find_country_name_of_ip(target_ip);

        // Then
        assert_eq!(result.unwrap(), ip_range_one.country_name);
    }

    #[test]
    fn test_find_country_name_of_ip_not_found() {
        // Given
        let ip_range_one = IPRange {
            start: 100,
            end: 16777215,
            country_code: String::from("HK"),
            country_name: String::from("Hong Kong"),
        };
        let ip_range_two = IPRange {
            start: 16777216,
            end: 16785407,
            country_code: String::from("CN"),
            country_name: String::from("China"),
        };

        let ip2location = IP2Location {
            ranges: vec![ip_range_one, ip_range_two],
        };
        let target_ip = 50;

        // When
        let result = ip2location.find_country_name_of_ip(target_ip);

        // Then
        assert_eq!(result.is_none(), true);
    }
}
