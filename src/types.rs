use serde::Deserialize;
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
