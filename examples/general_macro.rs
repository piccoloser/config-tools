fn main() {
    let config = config_tools::general_defaults! {
        "host" => "127.0.0.1",
        "port" => "8080",
    };

    config
        .save("general-manual.ini")
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
