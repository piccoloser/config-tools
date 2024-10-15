#![allow(unused, dead_code)]

use config_tools::{sectioned_defaults, Config, FromSection};

#[derive(Debug, FromSection)]
struct ServerSettings {
    address: String,
    port: u16,
    threads: u16,
}

fn main() {
    let config = Config::load_or_default("get-values.ini", || sectioned_defaults! {
        {
            "console" => "true",
            "logging" => "true",
        }
        ["Server"] {
            "address" => "127.0.0.1",
            "port" => "8080",
            "threads" => "4",
        }
    });

    let address = config
        .get(Some("Server"), "address")
        .unwrap();

    let port = config
        .get_as::<u16>(Some("Server"), "port")
        .unwrap();

    let threads: u16 = config.get_as(Some("Server"), "threads").unwrap();

    let server_settings = ServerSettings::from_section(&config.section("Server").unwrap()).unwrap();

    println!("Server Settings: {server_settings:#?}");
}
