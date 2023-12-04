use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

use crate::util::create_empty_file;

#[derive(Debug)]
pub struct Mod {
    pub metadata: ModMetadata,
    pub path: PathBuf,
}

impl Mod {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let metadata_path = path.join("metadata.xml");
        let metadata_contents = fs::read_to_string(metadata_path)?;
        let metadata = quick_xml::de::from_str(&metadata_contents)?;
        Ok(Self { metadata, path })
    }

    pub fn disable_path(&self) -> PathBuf {
        self.path.join("disable.it")
    }

    pub fn enabled(&self) -> bool {
        !self.disable_path().exists()
    }

    pub fn set_enabled(&mut self, enabled: bool) -> io::Result<()> {
        match enabled {
            true => fs::remove_file(self.disable_path()),
            false => create_empty_file(self.disable_path()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "metadata")]
pub struct ModMetadata {
    pub name: String,
    pub directory: String,
    pub id: u64,
    pub description: String,
    pub version: String,
    pub visibility: String,
    #[serde(rename = "tag")]
    pub tags: Vec<ModTag>,
}

#[derive(Debug, Deserialize)]
pub struct ModTag {
    #[serde(rename = "@id")]
    pub id: ModTagId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ModTagId {
    Lua,
    Items,
    #[serde(rename = "Active Items")]
    ActiveItems,
    Trinkets,
    Pills,
    Cards,
    Pickups,
    #[serde(rename = "Player Characters")]
    PlayerCharacters,
    Familiars,
    Babies,
    Rooms,
    Floors,
    Enemies,
    Bosses,
    Hazards,
    Challenges,
    Tweaks,
    Removals,
    Graphics,
    Shaders,
    #[serde(rename = "Sound Effects")]
    SoundEffects,
    Music,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub mods_path: PathBuf,
}

impl AppConfig {
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    pub fn load() -> anyhow::Result<Self> {
        if let Some(path) = directories::BaseDirs::new() {
            let path = path.config_dir().join("IcyIsaacModManager");
            let config_path = path.join("config.json");
            let config_contents = fs::read_to_string(config_path)?;
            let config = serde_json::from_str(&config_contents)?;
            Ok(config)
        } else {
            Err(anyhow!("Cannot load config: directory somehow missing"))
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(path) = directories::BaseDirs::new() {
            let path = path.config_dir().join("IcyIsaacModManager");
            if !path.exists() {
                fs::create_dir(&path)?;
            }
            let config_path = path.join("config.json");
            let config = serde_json::to_string_pretty(self)?;
            fs::write(config_path, config)?;
            Ok(())
        } else {
            Err(anyhow!("Cannot save config: directory somehow missing"))
        }
    }
}
