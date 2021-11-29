use std::collections::HashMap;

use crate::bans::LocatedBannedIp;

#[derive(PartialEq, Debug, Clone)]
pub struct CountryBans {
    pub country_name: String,
    pub numberofbans: i32,
}

pub struct Stats {
    banned_ips_sorted_by_bans_count: Vec<LocatedBannedIp>,
    bans_count_per_country: HashMap<String, i32>,
}

impl Stats {
    pub fn new(mut banned_ips: Vec<LocatedBannedIp>) -> Stats {
        banned_ips.sort_by(|a, b| b.numberofbans.cmp(&a.numberofbans));

        let mut bans_per_country: HashMap<String, i32> = HashMap::new();
        banned_ips.iter().for_each(|banned_ip| {
            let count = bans_per_country
                .entry(banned_ip.country_name.clone())
                .or_insert(0);
            *count += banned_ip.numberofbans;
        });

        Stats {
            banned_ips_sorted_by_bans_count: banned_ips,
            bans_count_per_country: bans_per_country,
        }
    }

    pub fn get_top_banned_ips(&self, limit: usize) -> Vec<LocatedBannedIp> {
        if limit >= self.banned_ips_sorted_by_bans_count.len() {
            return self.banned_ips_sorted_by_bans_count.to_vec();
        }

        self.banned_ips_sorted_by_bans_count[..limit].to_vec()
    }

    pub fn get_top_banned_countries(&self, limit: usize) -> Vec<CountryBans> {
        let mut country_bans: Vec<CountryBans> = self
            .bans_count_per_country
            .iter()
            .map(|(country_name, numberofbans)| CountryBans {
                country_name: country_name.clone(),
                numberofbans: numberofbans.clone(),
            })
            .collect();
        country_bans.sort_by(|a, b| b.numberofbans.cmp(&a.numberofbans));

        if limit >= country_bans.len() {
            return country_bans;
        }

        country_bans[..limit].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_top_banned_ips() {
        // Given
        let ip1 = LocatedBannedIp {
            ip: "115.238.245.4".to_string(),
            numberofbans: 8,
            country_name: "Germany".to_string(),
        };
        let ip2 = LocatedBannedIp {
            ip: "223.111.139.244".to_string(),
            numberofbans: 5,
            country_name: "France".to_string(),
        };
        let ip3 = LocatedBannedIp {
            ip: "61.184.247.11".to_string(),
            numberofbans: 20,
            country_name: "China".to_string(),
        };
        let ip4 = LocatedBannedIp {
            ip: "122.226.181.164".to_string(),
            numberofbans: 10,
            country_name: "China".to_string(),
        };
        let stats = Stats::new(vec![ip1.clone(), ip2.clone(), ip3.clone(), ip4.clone()]);

        // When
        let result = stats.get_top_banned_ips(4);

        // Then
        assert_eq!(result, vec![ip3, ip4, ip1, ip2]);
    }

    #[test]
    fn test_get_top_banned_countries() {
        // Given
        let ip1 = LocatedBannedIp {
            ip: "115.238.245.4".to_string(),
            numberofbans: 8,
            country_name: "Germany".to_string(),
        };
        let ip2 = LocatedBannedIp {
            ip: "223.111.139.244".to_string(),
            numberofbans: 5,
            country_name: "France".to_string(),
        };
        let ip3 = LocatedBannedIp {
            ip: "61.184.247.11".to_string(),
            numberofbans: 20,
            country_name: "China".to_string(),
        };
        let ip4 = LocatedBannedIp {
            ip: "122.226.181.164".to_string(),
            numberofbans: 10,
            country_name: "China".to_string(),
        };
        let stats = Stats::new(vec![ip1, ip2, ip3, ip4]);

        // When
        let result = stats.get_top_banned_countries(2);

        // Then
        assert_eq!(
            result,
            vec![
                CountryBans {
                    country_name: "China".to_string(),
                    numberofbans: 30,
                },
                CountryBans {
                    country_name: "Germany".to_string(),
                    numberofbans: 8,
                }
            ]
        );
    }
}
