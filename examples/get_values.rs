use config_tools::{sectioned_defaults, Config, FromSection, Section};

#[derive(Debug, FromSection)]
struct ServerSettings {
    address: String,
    port: u16,
    threads: u16,
}

fn main() {
    let config = Config::load_or_default(
        "get-values.ini",
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
        },
    );

    let console = config.get_as::<bool>(None, "console").unwrap();
    let log_level = config.get(None, "log_level").unwrap();

    let server_settings = ServerSettings::from_section(&config.section("Server").unwrap()).unwrap();

    println!(
        "General:\n    console={:?}\n    log_level={:?}",
        console, log_level
    );
    println!(
        "Server:\n    address={:?}\n    port={:?}\n    threads={:?}",
        server_settings.address, server_settings.port, server_settings.threads
    );
}
