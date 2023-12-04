use std::{fs, io};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{checkbox, column, container, row, scrollable, text, text_input},
    Alignment, Element, Length, Sandbox,
};
use rfd::FileDialog;

use crate::types::{AppConfig, Mod};

pub struct ModManager {
    mod_list: Vec<Mod>,
    state: AppState,
    config: AppConfig,
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(usize, bool),
    Refresh,
    EnableAll,
    DisableAll,

    // Config related entries
    OpenConfig,
    LeaveConfig,
    SaveConfig,
    SelectGamePath,
}

#[derive(Debug, Clone)]
pub enum AppState {
    ModList,
    Config(AppConfig),
}

impl ModManager {
    fn refresh_mods(&mut self) -> io::Result<()> {
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
        println!("{:#?}", mods);
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
        };
        let _ = manager.refresh_mods();
        manager
    }

    fn title(&self) -> String {
        String::from("Icy Isaac Mod Manager")
    }

    fn update(&mut self, message: Message) {
        // TODO: find better way to handle (and display) errors here
        // TODO: REALLY find a better way to handle errors
        // TODO: REALLY REALLY REALLY FIND A WAY TO ACTUALLY HANDLE ERRORS
        match message {
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
            // Config stuff
            Message::OpenConfig => self.state = AppState::Config(self.config.clone()),
            Message::LeaveConfig => self.state = AppState::ModList,
            Message::SaveConfig => {
                if let AppState::Config(temp_config) = &self.state {
                    self.config = temp_config.clone();
                    let _ = self.config.save();
                    println!("{:#?}", self.config);
                }
            }
            Message::SelectGamePath => {
                if let AppState::Config(temp_config) = &mut self.state {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        temp_config.mods_path = folder;
                    }
                }
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
                container(
                    row![
                        scroll,
                        column![
                            column![refresh, enable_all, disable_all]
                                .spacing(10)
                                .height(Length::Fill),
                            button("SETTINGS").on_press(Message::OpenConfig).width(120),
                        ]
                    ]
                    .spacing(10),
                )
                .padding(30)
                .into()
            }
            AppState::Config(temp_config) => {
                let back_button = button("RETURN").on_press(Message::LeaveConfig).width(120);
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
                let settings_col = column![game_path].height(Length::Fill);
                container(
                    column![header, settings_col, end_row]
                        .spacing(20)
                        .align_items(Alignment::Center),
                )
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
