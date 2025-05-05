use tempfile::NamedTempFile;

fn main() {
    let config = config_tools::sectioned_defaults! {
        { "console" => "true" }

        ["Application"] {
            "host" => "127.0.0.1",
            "port" => "8080",
        }

        ["Window"] {
            "width" => "720",
            "height" => "480",
        }
    };

    let tmp = NamedTempFile::new().unwrap();
    config
        .save(tmp.path())
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
