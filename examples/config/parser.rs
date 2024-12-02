/// this file showcases an example of how you can parse the config found in the Network Initialization File inside some structs, which can then be used to initialize the network
///
/// remember to add the Dependencies for toml to Cargo.toml:
///
/// [dependencies]
/// toml = "0.8.19"
use std::fs;
use wg_2024::config::Config;

fn main() {
    let config_data =
        fs::read_to_string("examples/config/config.toml").expect("Unable to read config file");
    // having our structs implement the Deserialize trait allows us to use the toml::from_str function to deserialize the config file into each of them
    let config: Config = toml::from_str(&config_data).expect("Unable to parse TOML");
    println!("{:#?}", config);
}
