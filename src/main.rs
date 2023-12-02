use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use iced::{
    widget::{checkbox, column, container, scrollable},
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
}

impl Sandbox for ModManager {
    type Message = Message;

    fn new() -> Self {
        const MOD_PATH: &str =
            "/ssd/SteamLibrary/steamapps/common/The Binding of Isaac Rebirth/mods/";
        let mut mods = Vec::new();
        if let Ok(dir) = fs::read_dir(Path::new(MOD_PATH)) {
            for entry in dir.flatten() {
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
        }
        Self { mod_list: mods }
    }

    fn title(&self) -> String {
        String::from("Icy Isaac Mod Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(i, b) => self.mod_list.get_mut(i).map(|m| m.set_enabled(b)),
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
        container(scroll).padding(20).into()
    }
}
