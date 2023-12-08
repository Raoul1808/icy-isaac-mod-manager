use std::{collections::HashMap, fmt::Display, fs};

use anyhow::{anyhow, Result};

use crate::{types::ModProfile, util::get_config_dir};

pub struct ModProfileManager {
    current_profile: i32,
    mod_profiles: HashMap<i32, ModProfile>,
    pub profile_states: Vec<ModProfileState>,  // These are public because iced
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
        if self.mod_profiles.contains_key(&id) {
            self.current_profile = id;
        }
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

#[cfg(test)]
mod test {
    use crate::types::ModProfile;

    use super::{ModProfileState, ModProfileManager};

    fn profile_state(id: i32, name: &str) -> ModProfileState {
        ModProfileState { id, name: name.to_string() }
    }

    #[test]
    fn next_free_id() {
        let mut manager = ModProfileManager::default();
        assert_eq!(manager.get_next_free_id(), 1, "There are no profiles, the next free ID should be 1");

        manager.create_empty_profile("Test Profile".to_string());
        assert_eq!(manager.get_next_free_id(), 2, "There is a single profile at ID 1, the next free ID should be 2");

        manager.create_empty_profile("Second Test Profile".to_string());
        manager.create_empty_profile("Third Test Profile".to_string());
        assert_eq!(manager.get_next_free_id(), 4, "There are 3 profiles with IDs 1, 2 and 3, the next free ID should be 4");

        manager.current_profile = 2;
        manager.delete_current_profile();
        assert_eq!(manager.get_next_free_id(), 2, "There are 2 profiles with IDs 1 and 3, the next free ID should be 2");

        manager.current_profile = 1;
        manager.delete_current_profile();
        assert_eq!(manager.get_next_free_id(), 1, "There is 1 profile with ID 3, the next free ID should be 1");
    }

    #[test]
    fn update_selected_profile() {
        let mut manager = ModProfileManager::default();

        manager.create_empty_profile("Test Profile".to_string());
        manager.create_empty_profile("Second Test Profile".to_string());
        manager.create_empty_profile("Third Test Profile".to_string());
        let state = profile_state(3, "Third Test Profile");
        assert_eq!(manager.current_profile, 3, "The current profile ID should be 3");
        assert_eq!(manager.current_profile_state, Some(state), "The current profile state should be ID 3 and name \"Third Test Profile\"");

        manager.update_selected_profile(2);
        let state = profile_state(2, "Second Test Profile");
        assert_eq!(manager.current_profile, 2, "The current profile ID should be 2");
        assert_eq!(manager.current_profile_state, Some(state.clone()), "The current profile state should be ID 2 and name \"Second Test Profile\"");

        manager.update_selected_profile(4);
        assert_eq!(manager.current_profile, 2, "The current profile ID should still be 2 after updating it to 4");
        assert_eq!(manager.current_profile_state, Some(state), "The current profile state should still be ID 2 and name \"Second Test Profile\" after updating the ID to 4");
    }

    #[test]
    fn get_current_profile() {
        let mut manager = ModProfileManager::default();
        manager.create_empty_profile("Test Profile".to_string());
        let expected_profile = ModProfile { name: "Test Profile".to_string(), enabled_mods: Vec::new() };
        let current_profile = manager.get_current_profile().unwrap();
        assert_eq!(current_profile.name, expected_profile.name);
        assert_eq!(current_profile.enabled_mods, expected_profile.enabled_mods);
    }

    #[test]
    fn update_current_profile() {
        let mut manager = ModProfileManager::default();
        
        manager.create_empty_profile("Test Profile".to_string());
        let current_profile = manager.get_current_profile().unwrap();
        assert!(current_profile.enabled_mods.is_empty(), "A new profile should have no enabled mods");

        let enabled_mods = vec![69, 420, 727, 996, 1116];
        manager.update_current_profile(enabled_mods.clone());
        let current_profile = manager.get_current_profile().unwrap();
        assert_eq!(current_profile.enabled_mods, enabled_mods, "Updating the profile should have the given enabled mods");

        let enabled_mods = vec![1, 2, 3];
        manager.update_current_profile(enabled_mods.clone());
        let current_profile = manager.get_current_profile().unwrap();
        assert_eq!(current_profile.enabled_mods, enabled_mods, "Updating the profile should replace the previous enabled mods vector");
    }

    #[test]
    fn delete_current_profile() {
        let mut manager = ModProfileManager::default();

        manager.create_empty_profile("Test Profile".to_string());
        assert_eq!(manager.current_profile, 1, "Current profile index should be 1");
        assert_eq!(manager.mod_profiles.len(), 1, "Mod Profiles length should be 1");

        manager.delete_current_profile();
        assert_eq!(manager.current_profile, 0, "Current profile index should be 0");
        assert_eq!(manager.mod_profiles.len(), 0, "Mod Profiles length should be 0");

        manager.create_empty_profile("Second Test Profile".to_string());
        manager.create_empty_profile("Third Test Profile".to_string());
        assert_eq!(manager.current_profile, 2, "Current profile index should be 2");
        assert_eq!(manager.mod_profiles.len(), 2, "Mod Profiles length should be 2");

        manager.delete_current_profile();
        assert_eq!(manager.current_profile, 0, "Current profile index should be 0");
        assert_eq!(manager.mod_profiles.len(), 1, "Mod Profiles length should be 1");
    }

    #[test]
    fn create_empty_profile() {
        let mut manager = ModProfileManager::default();

        manager.create_empty_profile("Test Profile".to_string());
        assert_eq!(manager.mod_profiles.len(), 1, "Mod Profiles length should be 1");
        assert_eq!(manager.current_profile, 1, "Current profile index should be 1");

        let profile = manager.mod_profiles.get(&1).unwrap();
        assert_eq!(profile.name, "Test Profile", "Check the new profile is named correctly");
        assert!(profile.enabled_mods.is_empty(), "Check the new profile has no enabled mods");
    }
}
