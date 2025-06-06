#[derive(Default,Clone)]
pub struct Chat {
    pub prompt: String,
    pub answer: String,
}

pub enum Selected {
    Prompt,
    Answer,
}

pub enum Mode {
    Main,
    Chat,
    Send,
    Exiting,
}

pub struct App {
    pub mode: Mode,
    pub is_editing: bool,
    pub selected: Selected,
    pub chat: Chat,
    pub chats: Vec<Chat>,
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::Main,
            is_editing: false,
            selected: Selected::Prompt,
            chat: Chat::default(),
            chats: vec![],
        }
    }
}
