fn main() {
    let config = config_tools::sectioned_defaults! {
        {
            "console" => "true"
        }

        ["Application"] {
            "host" => "127.0.0.1",
            "port" => "8080",
        }

        ["Window"] {
            "width" => "720",
            "height" => "480",
        }
    };

    config
        .save("sectioned-macro.ini")
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
