use std::{collections::HashMap, fmt::Display, fs};

use anyhow::{anyhow, Result};

use crate::{types::ModProfile, util::get_config_dir};

pub struct ModProfileManager {
    pub current_profile: i32,
    mod_profiles: HashMap<i32, ModProfile>,
    pub profile_states: Vec<ModProfileState>,
    pub current_profile_state: Option<ModProfileState>,
}

impl Default for ModProfileManager {
    fn default() -> Self {
        let default_profile = Self::get_default_profile();
        Self {
            current_profile: 0,
            mod_profiles: HashMap::new(),
            profile_states: Vec::new(),
            current_profile_state: Some(default_profile.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModProfileState {
    pub id: i32,
    pub name: String,
}

impl Display for ModProfileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ModProfileManager {
    fn get_default_profile() -> ModProfileState {
        ModProfileState {
            id: 0,
            name: "<default>".to_string(),
        }
    }

    pub fn load() -> Result<Self> {
        if let Some(path) = get_config_dir() {
            let profiles_path = path.join("profiles.json");
            let profiles_contents = fs::read_to_string(profiles_path)?;
            let mod_profiles = serde_json::from_str(&profiles_contents)?;
            let mut s = Self {
                current_profile: 0,
                mod_profiles,
                ..Default::default()
            };
            s.update_state();
            Ok(s)
        } else {
            Err(anyhow!(
                "Cannot load mod profiles: directory somehow missing"
            ))
        }
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = get_config_dir() {
            if !path.exists() {
                fs::create_dir(path.clone())?;
            }
            let profiles_path = path.join("profiles.json");
            let profiles_contents = serde_json::to_string_pretty(&self.mod_profiles)?;
            fs::write(profiles_path, profiles_contents)?;
            Ok(())
        } else {
            Err(anyhow!(
                "Cannot save mod profiles: directory somehow missing"
            ))
        }
    }

    pub fn get_current_profile(&self) -> Option<&ModProfile> {
        self.mod_profiles.get(&self.current_profile)
    }

    pub fn get_current_profile_mut(&mut self) -> Option<&mut ModProfile> {
        self.mod_profiles.get_mut(&self.current_profile)
    }

    pub fn create_empty_profile(&mut self, name: String) {
        let id = self.get_next_free_id();
        let mod_profile = ModProfile {
            name,
            enabled_mods: Vec::new(),
        };
        self.mod_profiles.insert(id, mod_profile);
        self.current_profile = id;
        self.update_state();
    }

    pub fn update_current_profile(&mut self, enabled: Vec<u64>) {
        if let Some(profile) = self.get_current_profile_mut() {
            profile.enabled_mods = enabled;
        }
    }

    pub fn update_selected_profile(&mut self, id: i32) {
        self.current_profile = id;
        if let Some(profile_state) = self
            .profile_states
            .iter()
            .find(|s| s.id == self.current_profile)
        {
            self.current_profile_state = Some(profile_state.clone());
        }
    }

    pub fn delete_current_profile(&mut self) {
        self.mod_profiles.remove(&self.current_profile);
        self.current_profile = 0;
        self.update_state();
    }

    fn get_next_free_id(&self) -> i32 {
        (1..i32::MAX)
            .find(|key| !self.mod_profiles.contains_key(key))
            .unwrap_or(1)
    }

    fn update_state(&mut self) {
        self.profile_states = self
            .mod_profiles
            .iter()
            .map(|(i, p)| ModProfileState {
                id: *i,
                name: p.name.clone(),
            })
            .collect();
        self.profile_states.insert(0, Self::get_default_profile());
        self.current_profile_state = Some(
            self.profile_states
                .iter()
                .find(|s| s.id == self.current_profile)
                .cloned()
                .unwrap_or(Self::get_default_profile()),
        );
    }
}
