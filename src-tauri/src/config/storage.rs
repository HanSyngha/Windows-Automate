// Configuration storage

use crate::commands::config::AppConfig;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Get config file path
fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("automate");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("config.json"))
}

/// Load configuration from file
pub fn load_config() -> Result<AppConfig> {
    let path = get_config_path()?;

    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        // Return default config if file doesn't exist
        Ok(AppConfig::default())
    }
}

/// Save configuration to file
pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}
