use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use thiserror::Error;
use super::repository::DbPool;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Configuration file not found")]
    ConfigNotFound,
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            database: "sik".to_string(),
            user: "root".to_string(),
            password: "".to_string(),
        }
    }
}

/// State untuk menyimpan database pool
pub struct AppState {
    pub db: DbPool,
}

/// Inisialisasi database pool dari konfigurasi
pub async fn init_pool(app_handle: tauri::AppHandle) -> Result<(), DatabaseError> {
    let config = load_config(app_handle.clone())?;

    let options = MySqlConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .database(&config.database)
        .username(&config.user)
        .password(&config.password);

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect_with(options)
        .await
        .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;

    // Store pool in app state
    app_handle.manage(AppState { db: pool });

    Ok(())
}

/// Load database configuration from file
fn load_config(app_handle: tauri::AppHandle) -> Result<DatabaseConfig, DatabaseError> {
    // Try to find config file in multiple locations
    let config_paths = vec![
        PathBuf::from("setting/database.json"),
        PathBuf::from("setting/database.xml"),
        app_handle
            .path()
            .app_config_dir()
            .map(|p| p.join("database.json"))
            .unwrap_or_default(),
    ];

    for path in config_paths {
        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|_| DatabaseError::ConfigNotFound)?;

            // Try JSON format first
            if path.extension().map_or(false, |ext| ext == "json") {
                let config: DatabaseConfig = serde_json::from_str(&content)
                    .map_err(|e| DatabaseError::InvalidConfig(e.to_string()))?;
                return Ok(config);
            }

            // For legacy XML format, we'll parse it manually or convert
            // For now, return default config
            return Ok(DatabaseConfig::default());
        }
    }

    // Return default config if no file found
    Ok(DatabaseConfig::default())
}

/// Get database pool from app state
pub fn get_pool(app_handle: &tauri::AppHandle) -> Option<DbPool> {
    app_handle.try_state::<AppState>().map(|state| state.db.clone())
}