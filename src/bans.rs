use rusqlite::{Connection, Result};

#[derive(PartialEq, Debug)]
pub struct BannedIp {
    pub ip: String,
    pub numberofbans: i32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LocatedBannedIp {
    pub ip: String,
    pub numberofbans: i32,
    pub country_name: String,
}

pub fn get_banned_ips(connection: &Connection) -> Result<Vec<BannedIp>, rusqlite::Error> {
    let mut stmt = connection
        .prepare("SELECT ip, count(ip) AS count FROM bans GROUP BY ip ORDER BY count DESC")?;
    let bans = stmt.query_map([], |row| {
        Ok(BannedIp {
            ip: row.get(0)?,
            numberofbans: row.get(1)?,
        })
    })?;

    let result: Vec<BannedIp> = bans.filter_map(Result::ok).collect();

    Ok(result)
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
        let result = get_banned_ips(&connection);

        // Then
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            result.unwrap(),
            vec![
                BannedIp {
                    ip: String::from("182.54.178.52"),
                    numberofbans: 2,
                },
                BannedIp {
                    ip: String::from("126.1.16.32"),
                    numberofbans: 1,
                }
            ]
        );
    }
}
