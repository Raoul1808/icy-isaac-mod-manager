use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use iced::{
    widget::{button, checkbox, column, container, row, scrollable},
    Element, Sandbox, Settings,
};

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}

fn create_empty_file(path: PathBuf) -> io::Result<()> {
    let _ = File::create(path)?;
    Ok(())
}

struct Mod {
    name: String,
    path: PathBuf,
}

impl Mod {
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
                let p = String::from(
                    path.file_name()
                        .unwrap_or(path.as_os_str())
                        .to_string_lossy(),
                );
                mods.push(Mod { name: p, path });
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
                    checkbox(m.name.to_owned(), m.enabled(), move |b| {
                        Message::Toggle(i, b)
                    })
                    .into()
                })
                .collect(),
        )
        .spacing(10)
        .padding(20);
        let scroll = scrollable(mod_list);
        let refresh = button("Refresh List").on_press(Message::Refresh);
        let enable_all = button("Enable All").on_press(Message::EnableAll);
        let disable_all = button("Disable All").on_press(Message::DisableAll);
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
