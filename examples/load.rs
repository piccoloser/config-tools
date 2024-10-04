use config_tools::Config;

fn main() {
    let config = Config::load("config-sectioned.ini").expect("Failed to load config file");
    println!("{config:#?}");
}
