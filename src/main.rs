#[macro_use]
extern crate log;
extern crate simplelog;

mod config;
mod database;
mod inspection;

use crate::database::Database;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use config::Config;
use inspection::Repetition;
use simplelog::*;

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )])
    .unwrap();
    //read configuration file toml (Should leave configurable for sysadmin)
    //  port should be configurable
    //  email should be configurable
    let config = Config::load();
    //read file db
    let connection = Database::open("data.db");
    // let connection =
    //     sqlite::open("data.db").expect("Failed to load inspections database (data.db)");
    let inspections = connection.get_inspections();

    println!("{:#?}", inspections);

    let utc: DateTime<Utc> = Utc::now();
    println!(
        "{:#?}",
        inspections[0].date - utc.with_timezone(&FixedOffset::east(2 * 3600))
    );

    //(zabbix should only check if program exists to verify it working correctly)
    //control through web gui
    //send email
    println!("Hello, world!");
}
