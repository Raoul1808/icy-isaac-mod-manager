use std::fs;

use anyhow::anyhow;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{checkbox, column, container, pick_list, row, scrollable, text, text_input},
    Alignment, Element, Length, Sandbox,
};
use rfd::FileDialog;

use crate::types::{AppConfig, Mod, Theme};

pub struct ModManager {
    mod_list: Vec<Mod>,
    state: AppState,
    config: AppConfig,
    current_theme: Option<Theme>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Mod list entries
    Toggle(usize, bool),
    Refresh,
    EnableAll,
    DisableAll,

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
}

impl Sandbox for ModManager {
    type Message = Message;

    fn new() -> Self {
        let mut manager = Self {
            mod_list: Default::default(),
            state: AppState::ModList,
            config: AppConfig::load_or_default(),
            current_theme: None,
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
                let refresh = button("REFRESH").on_press(Message::Refresh).width(120);
                let enable_all = button("ENABLE ALL").on_press(Message::EnableAll).width(120);
                let disable_all = button("DISABLE ALL")
                    .on_press(Message::DisableAll)
                    .width(120);
                let top_buttons = column![refresh, enable_all, disable_all]
                    .spacing(10)
                    .height(Length::Fill);
                let settings_button = button("SETTINGS").on_press(Message::OpenConfig).width(120);
                let about_button = button("ABOUT").on_press(Message::OpenAbout).width(120);
                let bottom_buttons = column![settings_button, about_button].spacing(10);
                container(row![scroll, column![top_buttons, bottom_buttons],].spacing(10))
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
