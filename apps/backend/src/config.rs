use serde::Deserialize;
use std::fs;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};

pub const SESSION_COOKIE_NAME: &str = "session_id";
pub const SESSION_DURATION_HOURS: i64 = 24;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub url: UrlConfig,
    pub vault: PathBuf,
    pub cors: CorsConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UrlConfig {
    pub base: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut config: Config = serde_yaml::from_str(&content)?;

        let config_dir = path.parent().unwrap_or_else(|| Path::new("."));

        if config.vault.is_relative() {
            config.vault = config_dir.join(&config.vault);
        }

        if let Some(db_path_str) = config.database.url.strip_prefix("sqlite://")
            && db_path_str != ":memory:"
            && let db_path = Path::new(db_path_str)
            && db_path.is_relative()
        {
            let new_db_path = config_dir.join(db_path);
            config.database.url = format!("sqlite://{}", new_db_path.display());
        }

        // Validate configuration immediately
        config
            .validate()
            .map_err(Box::<dyn std::error::Error>::from)?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if !self.vault.exists() {
            return Err(format!("Vault directory {:?} does not exist", self.vault));
        }
        Ok(())
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server.host, self.server.port)
    }

    pub fn audiobooks_dir(&self) -> PathBuf {
        self.vault.join("audiobooks")
    }
}
