use config_tools::{sectioned_defaults, Config};
use tempfile::NamedTempFile;

fn main() {
    let tmp = NamedTempFile::new().unwrap();

    // If you want to handle errors manually, use Config::load() instead.
    // Returns Result<config_tools::Config, config_tools::Error>
    // let config = Config::load(tmp);

    // Load and use defaults on failure
    let config = Config::load_or_default(
        tmp.path(),
        sectioned_defaults! {
                {
                    "host" => "127.0.0.1",
                    "port" => "8080",
                }
        },
    );

    // If you need to know whether or not defaults were used, call `load_or_default_outcome()` instead:
    // let outcome = Config::load_or_default_outcome(
    //     tmp.path(),
    //     sectioned_defaults! {
    //         {
    //             "host" => "127.0.0.1",
    //             "port" => "8080",
    //         }
    //     },
    // );
    //
    // if outcome.used_default() {
    //     println!("Using default config!");
    // }
    //
    // let config = outcome.into_inner();

    config.save(tmp.path()).expect("Failed to save config.");

    println!("{config:#?}");
}
