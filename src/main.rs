use iced::{Sandbox, Settings};

mod types;
mod ui;
mod util;

use crate::ui::ModManager;

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}
