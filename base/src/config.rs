use anyhow::{anyhow, Result};
use config_rs::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

pub fn load_config<'a, T: Deserialize<'a>>() -> Result<T> {
    dotenv().expect("Failed to read .env file");

    Config::builder()
        .add_source(Environment::default().separator("_"))
        .build()?
        .try_deserialize()
        .map_err(|err| {
            anyhow!(
                "Unable to deserialize into config with type {} with error: {}",
                std::any::type_name::<T>(),
                err
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_load_config_success() {
        std::env::set_var("LOG_LEVEL", "info");
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("SERVER_URL", "localhost");

        let expected = MyConfig {
            log: Log { level: "info".to_string() },
            server: Server {
                port: 8080,
                url: "localhost".to_string()
            },
        };

        let actual = load_config::<MyConfig>().unwrap();
        assert_eq!(expected, actual);
    }
}