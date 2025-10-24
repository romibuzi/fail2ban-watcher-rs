use std::process;
use std::thread;
use std::time::Instant;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use clap::Parser;
use csv::ReaderBuilder;
use rusqlite::{Connection, OpenFlags};
use rust_embed::RustEmbed;

use crate::bans::{BannedIp, LocatedBannedIp};
use crate::ip2location::IP2Location;
use crate::stats::Stats;

mod bans;
mod ip2location;
mod ipconverter;
mod stats;

const IP2LOCATION_DATABASE: &str = "IP2LOCATION-LITE-DB1.CSV";

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Resources;

fn init_ip2location() -> Result<IP2Location, std::io::Error> {
    match Resources::get(IP2LOCATION_DATABASE) {
        Some(file) => {
            let mut reader = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file.data.as_ref());

            Ok(IP2Location::from_reader(&mut reader))
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not find `{}` file", IP2LOCATION_DATABASE),
        )),
    }
}

fn get_bans_from_fail2ban_db(fail2ban_db_path: String) -> Result<Vec<BannedIp>, rusqlite::Error> {
    Connection::open_with_flags(fail2ban_db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .and_then(|connection| bans::get_banned_ips(&connection))
}

fn locate_banned_ips(ip2location: &IP2Location, banned_ips: &[BannedIp]) -> Vec<LocatedBannedIp> {
    banned_ips
        .iter()
        .filter_map(|banned_ip| {
            let country = ipconverter::ipv4_to_u32(&banned_ip.ip)
                .ok()
                .and_then(|target_ip| ip2location.find_country_name_of_ip(target_ip));

            match country {
                Some(country_name) => Some(LocatedBannedIp {
                    ip: banned_ip.clone(),
                    country_name,
                }),
                None => {
                    log::warn!("Could not locate country of ip {}", &banned_ip.ip);
                    None
                }
            }
        })
        .collect()
}

fn display_top_banned_countries(stats: &Stats, display_limit: usize) {
    println!("\n{}", Yellow.paint("Top banned countries:"));
    for country in stats.get_top_banned_countries(display_limit) {
        println!("{} bans: {}", country.number_of_bans, country.country_name);
    }
}

fn display_top_banned_ips(stats: &Stats, display_limit: usize) {
    println!("\n{}", Yellow.paint("Top banned IPs:"));
    for ip in stats.get_top_banned_ips(display_limit) {
        println!("{}", ip);
    }
}

fn program(
    fail2ban_db_path: String,
    display_limit: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Blue.paint("Starting analysis"));
    let start = Instant::now();

    let init_ip2location_thread = thread::spawn(init_ip2location);
    let get_bans_thread = thread::spawn(|| get_bans_from_fail2ban_db(fail2ban_db_path));

    let ip2location = init_ip2location_thread.join().unwrap()?;
    let bans = get_bans_thread.join().unwrap()?;

    let located_banned_ips = locate_banned_ips(&ip2location, &bans);
    let stats = Stats::new(located_banned_ips);

    display_top_banned_ips(&stats, display_limit);
    display_top_banned_countries(&stats, display_limit);

    println!(
        "\n{}",
        Green.paint(format!(
            "Analysis completed in {:.2} seconds",
            start.elapsed().as_secs_f32()
        ))
    );

    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "fail2ban database to analyze",
        default_value = "/var/lib/fail2ban/fail2ban.sqlite3"
    )]
    fail2ban_db_path: String,

    #[arg(
        short,
        long,
        help = "number of elements to display",
        default_value = "10"
    )]
    display_limit: usize,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(error) = program(args.fail2ban_db_path, args.display_limit) {
        let error_msg = format!("Error while running program: {}", error);
        eprintln!("{}", Red.paint(error_msg));

        process::exit(1);
    }
}
