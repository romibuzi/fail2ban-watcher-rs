use std::collections::HashMap;
use std::fmt;

use crate::bans::LocatedBannedIp;

#[derive(PartialEq, Debug, Clone)]
pub struct CountryBans {
    pub country_name: String,
    pub number_of_bans: u32,
}

impl fmt::Display for CountryBans {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bans: {}", self.number_of_bans, self.country_name)
    }
}

pub struct Stats {
    banned_ips_sorted_by_bans_count: Vec<LocatedBannedIp>,
    countries_sorted_by_bans_count: Vec<CountryBans>,
}

impl Stats {
    pub fn new(mut banned_ips: Vec<LocatedBannedIp>) -> Self {
        let is_sorted =
            banned_ips.is_sorted_by(|a, b| a.banned.number_of_bans >= b.banned.number_of_bans);
        if !is_sorted {
            banned_ips.sort_unstable_by_key(|b| std::cmp::Reverse(b.banned.number_of_bans));
        }

        let countries_sorted_by_bans_count = number_of_bans_per_country(&banned_ips);

        Stats {
            banned_ips_sorted_by_bans_count: banned_ips,
            countries_sorted_by_bans_count,
        }
    }

    pub fn top_banned_ips(&self, limit: usize) -> &[LocatedBannedIp] {
        let end = limit.min(self.banned_ips_sorted_by_bans_count.len());
        &self.banned_ips_sorted_by_bans_count[..end]
    }

    pub fn top_banned_countries(&self, limit: usize) -> &[CountryBans] {
        let end = limit.min(self.countries_sorted_by_bans_count.len());
        &self.countries_sorted_by_bans_count[..end]
    }
}

fn number_of_bans_per_country(banned_ips: &[LocatedBannedIp]) -> Vec<CountryBans> {
    let mut number_of_bans_per_country: HashMap<&str, u32> = HashMap::new();

    for banned_ip in banned_ips {
        *number_of_bans_per_country
            .entry(&banned_ip.country_name)
            .or_default() += banned_ip.banned.number_of_bans;
    }

    let mut countries: Vec<CountryBans> = number_of_bans_per_country
        .into_iter()
        .map(|(country_name, number_of_bans)| CountryBans {
            country_name: country_name.to_string(),
            number_of_bans,
        })
        .collect();

    countries.sort_unstable_by(|a, b| {
        b.number_of_bans
            .cmp(&a.number_of_bans)
            .then_with(|| a.country_name.cmp(&b.country_name))
    });
    countries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BannedIp;

    fn located(ip: &str, number_of_bans: u32, country_name: &str) -> LocatedBannedIp {
        LocatedBannedIp {
            banned: BannedIp {
                ip: ip.to_string(),
                number_of_bans,
            },
            country_name: country_name.to_string(),
        }
    }

    #[test]
    fn test_top_banned_ips() {
        // Given
        let ip1 = located("115.238.245.4", 8, "Germany");
        let ip2 = located("223.111.139.244", 5, "France");
        let ip3 = located("61.184.247.11", 20, "China");
        let ip4 = located("122.226.181.164", 10, "China");
        let stats = Stats::new(vec![ip1.clone(), ip2.clone(), ip3.clone(), ip4.clone()]);

        // When
        let result = stats.top_banned_ips(4);

        // Then
        assert_eq!(result, vec![ip3, ip4, ip1, ip2]);
    }

    #[test]
    fn test_top_banned_countries() {
        // Given
        let ip1 = located("115.238.245.4", 8, "Germany");
        let ip2 = located("223.111.139.244", 5, "France");
        let ip3 = located("61.184.247.11", 20, "China");
        let ip4 = located("122.226.181.164", 10, "China");
        let ip5 = located("122.100.156.10", 30, "Canada");
        let stats = Stats::new(vec![ip1, ip2, ip3, ip4, ip5]);

        // When
        let result = stats.top_banned_countries(3);

        // Then
        assert_eq!(
            result,
            vec![
                CountryBans {
                    country_name: "Canada".to_string(),
                    number_of_bans: 30,
                },
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
