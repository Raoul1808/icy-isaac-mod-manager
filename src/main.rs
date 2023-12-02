use iced::{widget::text, Sandbox, Settings};

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}

struct ModManager;

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for ModManager {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Icy Isaac Mod Manager")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Message> {
        text("Hello mod manager with iced!").into()
    }
}
