use std::collections::HashMap;

use crate::bans::LocatedBannedIp;

#[derive(PartialEq, Debug, Clone)]
pub struct CountryBans {
    pub country_name: String,
    pub number_of_bans: u32,
}

pub struct Stats {
    banned_ips_sorted_by_bans_count: Vec<LocatedBannedIp>,
    countries_sorted_by_bans_count: Vec<CountryBans>,
}

impl Stats {
    pub fn new(mut banned_ips: Vec<LocatedBannedIp>) -> Stats {
        let mut number_of_bans_per_country = Stats::get_number_of_bans_per_country(&banned_ips);

        banned_ips.sort_unstable_by(|a, b| b.number_of_bans.cmp(&a.number_of_bans));
        number_of_bans_per_country.sort_unstable_by(|a, b| b.number_of_bans.cmp(&a.number_of_bans));

        Stats {
            banned_ips_sorted_by_bans_count: banned_ips,
            countries_sorted_by_bans_count: number_of_bans_per_country,
        }
    }

    fn get_number_of_bans_per_country(banned_ips: &[LocatedBannedIp]) -> Vec<CountryBans> {
        let mut number_of_bans_per_country: HashMap<&str, u32> = HashMap::new();

        for banned_ip in banned_ips {
            *number_of_bans_per_country
                .entry(&banned_ip.country_name)
                .or_insert(0) += banned_ip.number_of_bans;
        }

        number_of_bans_per_country
            .into_iter()
            .map(|(country_name, number_of_bans)| CountryBans {
                country_name: country_name.to_string(),
                number_of_bans,
            })
            .collect()
    }

    pub fn get_top_banned_ips(&self, limit: usize) -> &[LocatedBannedIp] {
        let end = limit.min(self.banned_ips_sorted_by_bans_count.len());
        &self.banned_ips_sorted_by_bans_count[..end]
    }

    pub fn get_top_banned_countries(&self, limit: usize) -> &[CountryBans] {
        let end = limit.min(self.countries_sorted_by_bans_count.len());
        &self.countries_sorted_by_bans_count[..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BannedIp;

    #[test]
    fn test_get_top_banned_ips() {
        // Given
        let ip1 = LocatedBannedIp {
            ip: BannedIp {
                ip: "115.238.245.4".to_string(),
                number_of_bans: 8,
            },
            country_name: "Germany".to_string(),
        };
        let ip2 = LocatedBannedIp {
            ip: BannedIp {
                ip: "223.111.139.244".to_string(),
                number_of_bans: 5,
            },
            country_name: "France".to_string(),
        };
        let ip3 = LocatedBannedIp {
            ip: BannedIp {
                ip: "61.184.247.11".to_string(),
                number_of_bans: 20,
            },
            country_name: "China".to_string(),
        };
        let ip4 = LocatedBannedIp {
            ip: BannedIp {
                ip: "122.226.181.164".to_string(),
                number_of_bans: 10,
            },
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
            ip: BannedIp {
                ip: "115.238.245.4".to_string(),
                number_of_bans: 8,
            },
            country_name: "Germany".to_string(),
        };
        let ip2 = LocatedBannedIp {
            ip: BannedIp {
                ip: "223.111.139.244".to_string(),
                number_of_bans: 5,
            },
            country_name: "France".to_string(),
        };
        let ip3 = LocatedBannedIp {
            ip: BannedIp {
                ip: "61.184.247.11".to_string(),
                number_of_bans: 20,
            },
            country_name: "China".to_string(),
        };
        let ip4 = LocatedBannedIp {
            ip: BannedIp {
                ip: "122.226.181.164".to_string(),
                number_of_bans: 10,
            },
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
                    number_of_bans: 30,
                },
                CountryBans {
                    country_name: "Germany".to_string(),
                    number_of_bans: 8,
                }
            ]
        );
    }
}
