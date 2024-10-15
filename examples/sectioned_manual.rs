use config_tools::Config;

fn main() {
    let config = Config::builder()
        .section("Server")
        .set("host", "127.0.0.1")
        .set("port", "8080")
        .section("Window")
        .set("width", "720")
        .set("height", "480")
        .general()
        .set("console", "true")
        .build();

    config
        .save("sectioned-manual.ini")
        .expect("Failed to save config.");

    println!("{:#?}", config);
}
