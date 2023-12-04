use iced::{Sandbox, Settings};

mod types;
mod ui;
mod util;

use crate::ui::ModManager;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}
