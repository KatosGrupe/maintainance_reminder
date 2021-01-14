#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
extern crate simplelog;

mod client;
mod config;
mod database;
mod inspection;
mod mail;

use crate::database::Database;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use config::Config;
use inspection::Repetition;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, Message, SendableEmail,
    SmtpClient, Transport,
};
use native_tls::{Protocol, TlsConnector};
use rocket_contrib::templates::Template;
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

    // let mut smtp_client = mail::SmtpClient::connect(config.email);
    // smtp_client
    //     .send_email(
    //         "Priminimas priežiūros darbams",
    //         "Reikia perdažyti stulpą",
    //         vec![EmailAddress::new("ignas@kata.lt".to_string()).unwrap()],
    //         EmailAddress::new("ignas@kata.lt".to_string()).unwrap(),
    //     )
    //     .unwrap();

    // println!("{:#?}", result);

    // let mut tls_builder = TlsConnector::builder();
    // tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
    // let tls_parameters = ClientTlsParameters::new("smtp.example.com".to_string(),
    // tls_builder.build().unwrap())

    // let mut mailer = SmtpClient::new((config.email.server.address, config.email.server.port))

    // let client = lettre::smtp::client::new_simple("kata.lt").credentials()
    //(zabbix should only check if program exists to verify it working correctly)
    //control through web gui
    //send email
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![client::index, client::edit])
        .launch();
}
