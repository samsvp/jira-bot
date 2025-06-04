#[derive(Default)]
pub struct Chat {
    prompt: String,
    answer: String,
}

pub enum Editing {
    Prompt,
    Answer,
}

pub enum Mode {
    Main,
    Chat(Chat),
    Send,
    Exiting,
}

pub struct App {
    pub mode: Mode,
    pub editing: Option<Editing>,
    pub chat: Chat,
    pub chats: Vec<Chat>,
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
            editing: None,
            chat: Chat::default(),
            chats: vec![],
        }
    }
}
