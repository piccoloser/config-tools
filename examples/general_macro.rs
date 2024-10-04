fn main() -> Result<(), config_tools::Error> {
    let mut config = config_tools::general_defaults! {
        "host" => "127.0.0.1",
        "port" => "8080",
    };

    config.update(None, "host", "0.0.0.0");

    config.save("config-general-macro.ini")
}
