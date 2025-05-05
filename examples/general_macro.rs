use tempfile::NamedTempFile;

fn main() {
    let config = config_tools::general_defaults! {
        "host" => "127.0.0.1",
        "port" => "8080",
    };

    let tmp = NamedTempFile::new().unwrap();
    config
        .save(tmp.path())
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
