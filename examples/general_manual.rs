use config_tools::Config;

fn main() {
    let config = Config::builder()
        .set("host", "127.0.0.1")
        .set("port", "8080")
        .build();

    config
        .save("general-manual.ini")
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
