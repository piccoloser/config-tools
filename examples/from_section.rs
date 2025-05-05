#![allow(dead_code)]
use config_tools::{sectioned_defaults, Config, FromSection, Section};
use tempfile::NamedTempFile;

#[derive(Debug, FromSection)]
struct ServerSettings {
    address: String,
    port: u16,
    threads: u16,
}

#[derive(Debug, FromSection)]
struct LdapSettings {
    host: String,
    domain: String,
}

fn main() {
    let config = Config::load_or_default(
        NamedTempFile::new().unwrap().path(),
        sectioned_defaults! {
                {
                    "console" => "true",
                    "log_level" => "info",
                }
                ["Server"] {
                    "address" => "127.0.0.1",
                    "port" => "8080",
                    "threads" => "4",
                }
                ["LDAP"] {
                    "host" => "ldap://localhost:389",
                    "domain" => "example.com",
                }
        },
    );

    let ldap_settings = LdapSettings::from_section(&config.section("LDAP").unwrap()).unwrap();
    let server_settings = ServerSettings::from_section(&config.section("Server").unwrap()).unwrap();

    println!("{ldap_settings:#?}");
    println!("{server_settings:#?}");
}
