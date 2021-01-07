#[macro_use]
extern crate log;
extern crate simplelog;

mod config;
mod inspection;

use chrono::prelude::*;
use config::Config;
use inspection::InspectionDate;
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
    let connection =
        sqlite::open("data.db").expect("Failed to load inspections database (data.db)");

    let mut cursor = connection
        .prepare(
            "
select i.id,
       i.repetition,
       coalesce(max(id.completed_on), '0000-01-01T00:00:00+00:00') as last_completion_time from inspections i
left join inspection_date id on i.id = id.inspection_id
group by i.id, i.repetition;
",
        )
        .expect("Failed to prepare query")
        .cursor();
    let mut inspections = vec![];

    while let Some(row) = cursor.next().unwrap() {
        inspections.push(InspectionDate {
            id: row[0].as_integer().unwrap(),
            repetition: Repetition::from_string(row[1].as_string().unwrap())
                .expect("Failed to parse"),
            date: match DateTime::parse_from_rfc3339(row[2].as_string().unwrap()) {
                Ok(value) => value,
                Err(error) => panic!(
                    "Failed to parse ({}): {}",
                    error,
                    row[2].as_string().unwrap()
                ),
            },
        })
    }

    println!("{:#?}", inspections);

    //(zabbix should only check if program exists to verify it working correctly)
    //control through web gui
    //send email
    println!("Hello, world!");
}
