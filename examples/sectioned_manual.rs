use config_tools::Config;

fn main() -> Result<(), config_tools::Error> {
    let config = Config::new()
        .section("Server")
        .set("host", "127.0.0.1")
        .set("port", "8080")
        .section("Window")
        .set("width", "720")
        .set("height", "480")
        .general()
        .set("console", "true")
        .build();

    config.save("config-sectioned-manual.ini")
}
