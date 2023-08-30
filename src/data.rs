use serenity::model::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GuildConfig {
    pub rules_role_id: Option<RoleId>
}

#[derive(Serialize, Deserialize)]
struct Config {
    guilds: HashMap<GuildId, GuildConfig>
}

const CONFIG_PATH: &str = "data.json";

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {    
    if let Ok(data) = fs::read(path) {
        let text = String::from_utf8(data)?;
        let config: Config = serde_json::from_str(&text)?;
        Ok(config)
    } else {
        Ok(
            Config {
                guilds: HashMap::new()
            }
        )
    }
}

fn write_config(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = serde_json::to_string(config)?;
    std::fs::write(path, text)?;
    Ok(())
}

pub fn get_guild_config(guild_id: &GuildId) -> Result<GuildConfig, Box<dyn std::error::Error>> {
    let config = read_config(CONFIG_PATH)?;
    if let Some(guild_config) = config.guilds.get(guild_id) {
        Ok(*guild_config)
    } else {
        Ok(
            GuildConfig { rules_role_id: None }
        )
    }
}

pub fn write_guild_config(guild_id: &GuildId, guild_config: &GuildConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = read_config(CONFIG_PATH).unwrap();
    config.guilds.insert(*guild_id, *guild_config);
    write_config(&config, CONFIG_PATH)?;
    Ok(())
}