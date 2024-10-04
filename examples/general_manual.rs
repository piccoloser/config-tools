use config_tools::Config;

fn main() -> Result<(), config_tools::Error> {
    let config = Config::new()
        .set("host", "127.0.0.1")
        .set("port", "8080")
        .build();

    config.save("config-general-manual.ini")
}
