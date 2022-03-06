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

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Resources;

fn init_ip2location() -> Result<IP2Location, std::io::Error> {
    let ip2location_resource = "ip2location.csv";
    match Resources::get(ip2location_resource) {
        Some(file) => {
            let mut reader = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file.data.as_ref());

            Ok(IP2Location::from_reader(&mut reader))
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not find `{}` file", ip2location_resource),
        )),
    }
}

fn get_bans_from_fail2ban_db(fail2ban_db_path: String) -> Result<Vec<BannedIp>, rusqlite::Error> {
    Connection::open_with_flags(fail2ban_db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .and_then(|connection| bans::get_banned_ips(&connection))
}

fn locate_banned_ips(ip2location: &IP2Location, banned_ips: &[BannedIp]) -> Vec<LocatedBannedIp> {
    let mut located_banned_ips: Vec<LocatedBannedIp> = Vec::with_capacity(banned_ips.len());

    for banned_ip in banned_ips {
        let country = ipconverter::ipv4_to_long(&banned_ip.ip)
            .ok()
            .and_then(|ip_long| ip2location.find_country_name_of_ip(ip_long));

        match country {
            Some(country_name) => located_banned_ips.push(LocatedBannedIp {
                ip: banned_ip.ip.clone(),
                numberofbans: banned_ip.numberofbans,
                country_name,
            }),

            None => log::warn!("Could not locate country of ip {}", &banned_ip.ip),
        }
    }

    located_banned_ips
}

fn display_top_banned_countries(stats: &Stats, number_of_elements_to_display: usize) {
    println!("\n{}", Yellow.paint("Top banned countries:"));
    for country in stats.get_top_banned_countries(number_of_elements_to_display) {
        println!("{} bans: {}", country.numberofbans, country.country_name);
    }
}

fn display_top_banned_ips(stats: &Stats, number_of_elements_to_display: usize) {
    println!("\n{}", Yellow.paint("Top banned IPs:"));
    for ip in stats.get_top_banned_ips(number_of_elements_to_display) {
        println!("{} bans: {} ({})", ip.numberofbans, ip.ip, ip.country_name);
    }
}

fn program(
    fail2ban_db_path: String,
    number_of_elements_to_displays: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Blue.paint("Starting analysis"));
    let start = Instant::now();

    let init_ip2location_thread = thread::spawn(init_ip2location);
    let get_bans_thread = thread::spawn(|| get_bans_from_fail2ban_db(fail2ban_db_path));

    let ip2location = init_ip2location_thread.join().unwrap()?;
    let bans = get_bans_thread.join().unwrap()?;

    let located_banned_ips = locate_banned_ips(&ip2location, &bans);
    let stats = Stats::new(located_banned_ips);

    display_top_banned_ips(&stats, number_of_elements_to_displays);
    display_top_banned_countries(&stats, number_of_elements_to_displays);

    let completion_msg = format!(
        "\nAnalysis completed in {:.2} seconds",
        start.elapsed().as_secs_f32()
    );
    println!("{}", Green.paint(completion_msg));

    Ok(())
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Romain A. <romain.ardiet@gmail.com>")]
struct Opts {
    #[clap(
        short,
        long,
        help = "fail2ban db to analyze",
        default_value = "/var/lib/fail2ban/fail2ban.sqlite3"
    )]
    fail2ban_db_path: String,

    #[clap(
        short,
        long,
        help = "number of elements to display",
        default_value = "10"
    )]
    nb_display: usize,
}

fn main() {
    env_logger::init();
    let opts: Opts = Opts::parse();

    if let Err(error) = program(opts.fail2ban_db_path, opts.nb_display) {
        let error_msg = format!("Error while running program: {}", error);
        println!("{}", Red.paint(error_msg));

        process::exit(1);
    }
}
