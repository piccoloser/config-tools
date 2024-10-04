fn main() -> Result<(), config_tools::Error> {
    let mut config = config_tools::sectioned_defaults! {
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

    config.update(Some("Application"), "host", "0.0.0.0");

    config.save("config-sectioned-macro.ini")
}
