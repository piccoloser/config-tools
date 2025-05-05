use config_tools::Config;
use tempfile::NamedTempFile;

fn main() {
    let config = Config::builder()
        .set("host", "127.0.0.1")
        .set("port", "8080")
        .build();

    let tmp = NamedTempFile::new().unwrap();
    config
        .save(tmp.path())
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
