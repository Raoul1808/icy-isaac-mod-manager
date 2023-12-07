use std::fs;

use anyhow::anyhow;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{checkbox, column, container, pick_list, row, scrollable, text, text_input},
    Alignment, Element, Length, Sandbox,
};
use rfd::FileDialog;

use crate::{
    manager::{ModProfileManager, ModProfileState},
    types::{AppConfig, Mod, Theme},
};

pub struct ModManager {
    mod_list: Vec<Mod>,
    state: AppState,
    config: AppConfig,
    current_theme: Option<Theme>,
    profile_manager: ModProfileManager,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Mod list entries
    Toggle(usize, bool),
    Refresh,
    EnableAll,
    DisableAll,

    // Quick mod profile management
    SelectProfile(ModProfileState),
    LoadProfile,
    SaveProfile,
    ManageProfiles,

    // Advanced profile management
    OnProfileNameEdit(String),
    CreateNewProfile,
    DeleteCurrentProfile,

    // Navigation
    OpenConfig,
    ReturnToModList,
    OpenAbout,

    // Config related entries
    SaveConfig,
    SelectGamePath,
    SwitchTheme(Theme),

    // Misc
    ActionOpen(String),
}

#[derive(Debug, Clone)]
pub enum AppState {
    ModList,
    Profiles { temp_profile_name: String },
    Config(AppConfig),
    About,
}

impl ModManager {
    fn refresh_mods(&mut self) -> anyhow::Result<()> {
        if self.config.mods_path.as_os_str().is_empty() || self.config.mods_path.is_relative() {
            return Err(anyhow!("Invalid mod path set!"));
        }
        let mut mods = Vec::new();
        for entry in fs::read_dir(&self.config.mods_path)?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                match Mod::from_path(path) {
                    Ok(m) => mods.push(m),
                    Err(e) => println!("Error loading mod: {e}"),
                }
            }
        }
        self.mod_list = mods;
        Ok(())
    }

    fn get_enabled_mod_ids(&self) -> Vec<u64> {
        self.mod_list
            .iter()
            .filter(|m| m.enabled())
            .map(|m| m.metadata.id)
            .collect()
    }
}

impl Sandbox for ModManager {
    type Message = Message;

    fn new() -> Self {
        let mut manager = Self {
            mod_list: Default::default(),
            state: AppState::ModList,
            config: AppConfig::load_or_default(),
            current_theme: None,
            profile_manager: ModProfileManager::load_or_default(),
        };
        manager.current_theme = Some(manager.config.theme);
        let _ = manager.refresh_mods();
        manager
    }

    fn title(&self) -> String {
        String::from("Icy Isaac Mod Manager")
    }

    fn theme(&self) -> iced::Theme {
        match self.current_theme.unwrap_or_default() {
            Theme::Dark => iced::Theme::Dark,
            Theme::Light => iced::Theme::Light,
        }
    }

    fn update(&mut self, message: Message) {
        // TODO: find better way to handle (and display) errors here
        // TODO: REALLY find a better way to handle errors
        // TODO: REALLY REALLY REALLY FIND A WAY TO ACTUALLY HANDLE ERRORS
        match message {
            // Mod list
            Message::Toggle(i, b) => {
                let _ = self.mod_list.get_mut(i).map(|m| m.set_enabled(b));
            }
            Message::Refresh => {
                let _ = self.refresh_mods();
            }
            Message::EnableAll => {
                for m in self.mod_list.iter_mut() {
                    let _ = m.set_enabled(true);
                }
            }
            Message::DisableAll => {
                for m in self.mod_list.iter_mut() {
                    let _ = m.set_enabled(false);
                }
            }
            // Mod profile management
            Message::SelectProfile(profile) => {
                self.profile_manager.update_selected_profile(profile.id);
                // self.profile_manager.current_profile_state = Some(profile);
                println!("{:#?}", self.profile_manager.current_profile_state);
            }
            Message::LoadProfile => {
                if let Some(profile) = self.profile_manager.get_current_profile() {
                    self.mod_list.iter_mut().for_each(|m| {
                        let enabled = profile.enabled_mods.contains(&m.metadata.id);
                        let _ = m.set_enabled(enabled);
                    });
                }
            }
            Message::SaveProfile => {
                let enabled_mods = self.get_enabled_mod_ids();
                self.profile_manager.update_current_profile(enabled_mods);
                let _ = self.profile_manager.save();
            }
            Message::ManageProfiles => {
                self.state = AppState::Profiles {
                    temp_profile_name: String::new(),
                };
            }
            // Advanced profile management
            Message::OnProfileNameEdit(name) => {
                if let AppState::Profiles { temp_profile_name } = &mut self.state {
                    *temp_profile_name = name;
                }
            }
            Message::CreateNewProfile => {
                if let AppState::Profiles { temp_profile_name } = &mut self.state {
                    self.profile_manager
                        .create_empty_profile(temp_profile_name.clone());
                    let _ = self.profile_manager.save();
                }
            }
            Message::DeleteCurrentProfile => {
                if let AppState::Profiles { .. } = &mut self.state {
                    self.profile_manager.delete_current_profile();
                    let _ = self.profile_manager.save();
                }
            }
            // Navigation stuff
            Message::OpenConfig => self.state = AppState::Config(self.config.clone()),
            Message::ReturnToModList => {
                self.state = AppState::ModList;
                self.current_theme = Some(self.config.theme);
            }
            Message::OpenAbout => self.state = AppState::About,
            // Config stuff
            Message::SaveConfig => {
                if let AppState::Config(temp_config) = &self.state {
                    self.config = temp_config.clone();
                    let _ = self.config.save();
                }
            }
            Message::SelectGamePath => {
                if let AppState::Config(temp_config) = &mut self.state {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        temp_config.mods_path = folder;
                    }
                }
            }
            Message::SwitchTheme(theme) => {
                if let AppState::Config(temp_config) = &mut self.state {
                    temp_config.theme = theme;
                }
                self.current_theme = Some(theme);
            }
            // Misc
            Message::ActionOpen(action) => {
                let _ = open::that_detached(action);
            }
        };
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::ModList => {
                let mod_list = column(
                    self.mod_list
                        .iter()
                        .enumerate()
                        .map(|(i, m)| {
                            checkbox(m.metadata.name.to_owned(), m.enabled(), move |b| {
                                Message::Toggle(i, b)
                            })
                            .into()
                        })
                        .collect(),
                )
                .spacing(10);
                let scroll = scrollable(mod_list).width(Length::Fill);
                let refresh = button("REFRESH").on_press(Message::Refresh).width(128);
                let enable_all = button("ENABLE ALL").on_press(Message::EnableAll).width(128);
                let disable_all = button("DISABLE ALL")
                    .on_press(Message::DisableAll)
                    .width(128);
                let top_buttons = column![refresh, enable_all, disable_all]
                    .spacing(10)
                    .height(Length::Fill);

                let profile_combo = pick_list(
                    &self.profile_manager.profile_states[..],
                    self.profile_manager.current_profile_state.clone(),
                    Message::SelectProfile,
                );
                let profile_load = button("LOAD PROFILE")
                    .on_press(Message::LoadProfile)
                    .width(128);
                let profile_save = button("SAVE PROFILE")
                    .on_press(Message::SaveProfile)
                    .width(128);
                let profile_new = button("MANAGE PROFILES")
                    .on_press(Message::ManageProfiles)
                    .width(128);
                let profile_buttons =
                    column![profile_combo, profile_new, profile_load, profile_save]
                        .spacing(10)
                        .height(Length::Fill);

                let settings_button = button("SETTINGS").on_press(Message::OpenConfig).width(128);
                let about_button = button("ABOUT").on_press(Message::OpenAbout).width(128);
                let bottom_buttons = column![settings_button, about_button].spacing(10);
                container(
                    row![
                        scroll,
                        column![top_buttons, profile_buttons, bottom_buttons],
                    ]
                    .spacing(10),
                )
                .padding(30)
                .into()
            }
            AppState::Profiles { temp_profile_name } => {
                let header_title = text("Profile Management")
                    .size(32)
                    .horizontal_alignment(Horizontal::Center);

                let profiles_label = text("Selected Profile")
                    .vertical_alignment(Vertical::Center)
                    .line_height(iced::widget::text::LineHeight::Relative(2.));
                let profiles_list = pick_list(
                    &self.profile_manager.profile_states[..],
                    self.profile_manager.current_profile_state.clone(),
                    Message::SelectProfile,
                );
                let profiles_row = row![profiles_label, profiles_list].spacing(10);

                let input_label = text("New Profile Name")
                    .vertical_alignment(Vertical::Center)
                    .line_height(iced::widget::text::LineHeight::Relative(2.));
                let input_field =
                    text_input("Type the name of a profile to create", temp_profile_name)
                        .on_input(Message::OnProfileNameEdit);
                let profile_create_row = row![input_label, input_field].spacing(10);
                let create_profile_button = button("CREATE PROFILE")
                    .width(150)
                    .on_press(Message::CreateNewProfile);
                let remove_profile_button = button("REMOVE PROFILE")
                    .width(150)
                    .on_press(Message::DeleteCurrentProfile);
                let profile_buttons_row =
                    row![create_profile_button, remove_profile_button].spacing(10);

                let options =
                    column![profiles_row, profile_create_row, profile_buttons_row].spacing(10);

                let back_button = button("RETURN")
                    .on_press(Message::ReturnToModList)
                    .width(120);
                let save_button = button("SAVE").on_press(Message::SaveProfile).width(120);
                let end_row = row![back_button, save_button].spacing(20);
                container(
                    column![header_title, options, end_row]
                        .spacing(30)
                        .align_items(Alignment::Center),
                )
                .padding(30)
                .into()
            }
            AppState::Config(temp_config) => {
                let back_button = button("RETURN")
                    .on_press(Message::ReturnToModList)
                    .width(120);
                let save_button = button("SAVE").on_press(Message::SaveConfig).width(120);
                let header = text("Application Settings")
                    .horizontal_alignment(Horizontal::Center)
                    .size(32);
                let end_row = row![back_button, save_button].spacing(10);
                let game_path_label = text("Path to game mods")
                    .vertical_alignment(Vertical::Center)
                    .line_height(iced::widget::text::LineHeight::Relative(2.));
                let game_path_field =
                    text_input("Path to game mods", temp_config.mods_path.to_str().unwrap());
                let game_path_button = button("BROWSE")
                    .on_press(Message::SelectGamePath)
                    .width(120);
                let game_path =
                    row![game_path_label, game_path_field, game_path_button].spacing(10);
                let theme_label = text("Theme")
                    .vertical_alignment(Vertical::Center)
                    .line_height(iced::widget::text::LineHeight::Relative(2.));
                let theme_pick =
                    pick_list(&Theme::ALL[..], self.current_theme, Message::SwitchTheme);
                let theme = row![theme_label, theme_pick].spacing(10);
                let settings_col = column![game_path, theme].spacing(10).height(Length::Fill);
                container(
                    column![header, settings_col, end_row]
                        .spacing(20)
                        .align_items(Alignment::Center),
                )
                .padding(30)
                .into()
            }
            AppState::About => {
                let title = text("Icy Isaac Mod Manager")
                    .size(32)
                    .horizontal_alignment(Horizontal::Center);
                let version_string = text(format!("Version {}", crate::APP_VERSION))
                    .size(12)
                    .horizontal_alignment(Horizontal::Center);
                let header_col = column![title, version_string]
                    .spacing(10)
                    .align_items(Alignment::Center);
                let back_button = button("BACK").on_press(Message::ReturnToModList).width(120);
                let intro_text =
                    text("Made with <3 by Mew.").horizontal_alignment(Horizontal::Center);
                let this_code = button("Source Code")
                    .on_press(Message::ActionOpen(crate::REPOSITORY.to_string()))
                    .style(iced::theme::Button::Secondary)
                    .width(240);
                let rust_button = button("Made in Rust")
                    .on_press(Message::ActionOpen("https://rust-lang.org/".to_string()))
                    .style(iced::theme::Button::Secondary)
                    .width(240);
                let iced_button = button("Made with Iced")
                    .on_press(Message::ActionOpen("https://iced.rs/".to_string()))
                    .style(iced::theme::Button::Secondary)
                    .width(240);
                let bunch_o_links = column![this_code, rust_button, iced_button]
                    .spacing(10)
                    .align_items(Alignment::Center);
                let middle_content = column![intro_text, bunch_o_links]
                    .spacing(50)
                    .padding(20)
                    .height(Length::Fill)
                    .align_items(Alignment::Center);
                container(
                    column![header_col, middle_content, back_button]
                        .spacing(20)
                        .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .center_x()
                .padding(30)
                .into()
            }
        }
    }
}

fn button(text: &str) -> iced::widget::Button<'_, Message> {
    iced::widget::button(
        iced::widget::text(text).horizontal_alignment(iced::alignment::Horizontal::Center),
    )
    .padding(5)
}
