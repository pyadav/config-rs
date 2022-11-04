use base::config::load_config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
    struct MyConfig {
        log: Log,
        server: Server,
    }

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Log {
    pub level: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Server {
    pub port: u16,
    pub url: String,
}
fn main() {
    let data = load_config::<MyConfig>().unwrap(); 
    println!("{:?}", data)
}