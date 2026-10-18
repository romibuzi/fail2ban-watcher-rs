use rusqlite::{Connection, Result};
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct BannedIp {
    pub ip: String,
    pub number_of_bans: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocatedBannedIp {
    pub banned: BannedIp,
    pub country_name: String,
}

impl fmt::Display for LocatedBannedIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} bans: {} ({})",
            self.banned.number_of_bans, self.banned.ip, self.country_name
        )
    }
}

impl BannedIp {
    pub fn located(self, country_name: String) -> LocatedBannedIp {
        LocatedBannedIp {
            banned: self,
            country_name,
        }
    }
}

pub fn banned_ips(connection: &Connection) -> Result<Vec<BannedIp>> {
    let mut stmt = connection
        .prepare("SELECT ip, COUNT(ip) AS count FROM bans GROUP BY ip ORDER BY count DESC")?;
    let bans = stmt
        .query_map([], |row| {
            Ok(BannedIp {
                ip: row.get(0)?,
                number_of_bans: row.get(1)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    Ok(bans)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_inmemory_connection() -> Connection {
        return Connection::open_in_memory().unwrap();
    }

    fn create_bans_table(connection: &Connection) {
        connection
            .execute(
                "create table if not exists bans (
                ip text
            )",
                [],
            )
            .unwrap();
    }

    fn insert_banned_ip(connection: &Connection, banned_ip: &str) {
        connection
            .execute("INSERT INTO bans (ip) values (?1)", &[banned_ip])
            .unwrap();
    }

    #[test]
    fn test_get_banned_ips() {
        // Given
        let connection = open_inmemory_connection();
        create_bans_table(&connection);
        insert_banned_ip(&connection, "126.1.16.32");
        insert_banned_ip(&connection, "182.54.178.52");
        insert_banned_ip(&connection, "182.54.178.52");

        // When
        let result = banned_ips(&connection);

        // Then
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                BannedIp {
                    ip: String::from("182.54.178.52"),
                    number_of_bans: 2,
                },
                BannedIp {
                    ip: String::from("126.1.16.32"),
                    number_of_bans: 1,
                }
            ]
        );
    }
}
