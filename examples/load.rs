use config_tools::Config;

fn main() {
    let filename = "config.ini"; // Change as needed
    let config = Config::load(filename).expect("Failed to load config file");
    println!("{config:#?}");
}
