use config_tools::{sectioned_defaults, Config};

fn main() {
    let filename = "load-file.ini";

    // If you want to handle errors manually, use Config::load() instead.
    // Returns Result<config_tools::Config, config_tools::Error>
    // let config = Config::load(filename);

    // Load and use defaults on failure
    let config = Config::load_or_default(filename, || {
        return sectioned_defaults! {
            {
                "host" => "127.0.0.1",
                "port" => "8080",
            }
        }
    });

    config.save(filename).expect("Failed to save config.");

    println!("{config:#?}");
}
