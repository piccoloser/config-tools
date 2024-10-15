use config_tools::{sectioned_defaults, Config};

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

    println!("{config:#?}\n---");

    let address = config
        .get(Some("Server"), "address")
        .unwrap();

    let port = config
        .get_as::<u16>(Some("Server"), "port")
        .unwrap();

    let threads: u16 = config.get_as(Some("Server"), "threads").unwrap();

    println!("
        Address: {address}
        Port: {port}
        Threads: {threads}
    ");
}
