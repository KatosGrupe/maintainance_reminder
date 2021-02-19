use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    server: ServerConfig,
    pub email: EmailConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    port: i32,
    listen: String,
}

#[derive(Deserialize)]
pub struct EmailConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub server: String,
}

impl Config {
    pub fn load() -> Config {
        use std::fs::File;
        use std::io::Read;
        let mut config_file =
            File::open("config.toml").expect("Error opening configuration file. (config.toml)");
        let mut config: String = String::new();
        config_file
            .read_to_string(&mut config)
            .expect("Error reading configuration file. (config.toml)");
        toml::from_str(&config).expect("Failed to parse configuration file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_read_default_toml_config() {
        let config = Config::load();
        assert_eq!(config.server.port, 21904);
        assert_eq!(config.server.listen, "127.0.0.1");
        assert_eq!(config.email.username, "test@test.org");
        assert_eq!(config.email.password, "SecurePassword1");
        assert_eq!(config.email.name, "SomeName");
        assert_eq!(config.email.server, "mail.server.org");
        assert_eq!(config.email.port, 25)
    }
}
