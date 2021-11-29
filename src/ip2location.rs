use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
struct IPRange {
    start: i64,
    end: i64,
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

    pub fn find_country_name_of_ip(&self, target_ip_long: i64) -> Option<String> {
        let mut low: i32 = 0;
        let mut high: i32 = self.ranges.len() as i32 - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let range = self.ranges.get(mid as usize).unwrap();

            if range.start <= target_ip_long && target_ip_long <= range.end {
                return Some(range.country_name.clone());
            }

            // Search values that are greater than range -> to right of current mid_index
            if range.start < target_ip_long {
                low = mid + 1;
            }

            // Search values that are less than range -> to the left of current mid_index
            if range.start > target_ip_long {
                high = mid - 1;
            }
        }

        None
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
