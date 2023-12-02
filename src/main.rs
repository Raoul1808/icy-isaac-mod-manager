use iced::{
    widget::{checkbox, column},
    Element, Sandbox, Settings,
};

fn main() -> iced::Result {
    ModManager::run(Settings::default())
}

struct ModManager {
    mod_list: Vec<(bool, String)>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(usize, bool),
}

impl Sandbox for ModManager {
    type Message = Message;

    fn new() -> Self {
        let mut mods = Vec::new();
        for i in 0..10 {
            mods.push((rand::random::<i32>() % 2 == 1, format!("Testing {i}")));
        }
        Self { mod_list: mods }
    }

    fn title(&self) -> String {
        String::from("Icy Isaac Mod Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle(i, b) => self.mod_list.get_mut(i).and_then(|m| Some(m.0 = b)),
        };
    }

    fn view(&self) -> Element<'_, Message> {
        column(
            self.mod_list
                .iter()
                .enumerate()
                .map(|(i, (e, m))| checkbox(m, *e, move |b| Message::Toggle(i, b)).into())
                .collect(),
        )
        .spacing(10)
        .padding(20)
        .into()
    }
}
