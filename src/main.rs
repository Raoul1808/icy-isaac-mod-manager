use std::{fs, io};

use iced::{
    widget::{checkbox, column, container, row, scrollable},
    Element, Sandbox, Settings,
};

mod types;
mod util;

use crate::types::Mod;

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}

struct ModManager {
    mod_list: Vec<Mod>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(usize, bool),
    Refresh,
    EnableAll,
    DisableAll,
}

impl ModManager {
    fn refresh_mods(&mut self) -> io::Result<()> {
        const MOD_PATH: &str =
            "/ssd/SteamLibrary/steamapps/common/The Binding of Isaac Rebirth/mods/";
        let mut mods = Vec::new();
        for entry in fs::read_dir(MOD_PATH)?.flatten() {
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
        };
    }

    fn view(&self) -> Element<'_, Message> {
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
        .spacing(10)
        .padding(20);
        let scroll = scrollable(mod_list);
        let refresh = button("REFRESH").on_press(Message::Refresh).width(120);
        let enable_all = button("ENABLE ALL").on_press(Message::EnableAll).width(120);
        let disable_all = button("DISABLE ALL")
            .on_press(Message::DisableAll)
            .width(120);
        container(
            row![
                scroll,
                column![refresh, enable_all, disable_all].spacing(10)
            ]
            .spacing(10),
        )
        .padding(20)
        .into()
    }
}

fn button(text: &str) -> iced::widget::Button<'_, Message> {
    iced::widget::button(
        iced::widget::text(text).horizontal_alignment(iced::alignment::Horizontal::Center),
    )
    .padding(5)
}
