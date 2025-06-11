use crossterm::event::{self, KeyCode, KeyEvent};

use super::app::{App, Mode, Selected};

pub fn handle_input(key: &KeyEvent, cols: u16, app: &mut App) -> bool {
    if key.kind == event::KeyEventKind::Release {
        return false;
    }

    match &mut app.mode {
        Mode::Main => match key.code {
            KeyCode::Char('q') => {
                app.mode = Mode::Exiting;
            }
            KeyCode::Char('i') => {
                app.is_editing = true;
                app.mode = Mode::Chat;
            }
            KeyCode::Tab => {
                app.selected = match app.selected {
                    Selected::Prompt => Selected::Answer,
                    Selected::Answer => Selected::Prompt,
                };
            }
            _ => {}
        }
        Mode::Chat => match key.code {
            KeyCode::Esc => {
                app.mode = Mode::Main;
                app.is_editing = false;
            }
            KeyCode::Down => {
                match &mut app.selected {
                    Selected::Prompt => app.chat.prompt.current_scroll += 1,
                    Selected::Answer => app.chat.answer.current_scroll += 1
                }
            }
            KeyCode::Up => {
                let text = match &app.selected {
                    Selected::Prompt => &mut app.chat.prompt,
                    Selected::Answer => &mut app.chat.answer
                };

                if text.current_scroll > 0 {
                    text.current_scroll -= 1;
                }
            }
            code => {
                let text = match &app.selected {
                    Selected::Prompt => &mut app.chat.prompt,
                    Selected::Answer => &mut app.chat.answer
                };

                match code {
                    KeyCode::Backspace => { text.text.pop(); }
                    KeyCode::Enter => text.text.push('\n'),
                    KeyCode::Char(c) => text.text.push(c),
                    _ => {}
                }

                let mut new_line = String::new();
                let current_line = &text.text;
                let mut current_index: usize = 0;
                let max_len = 15 as usize;//cols as usize;
                loop {
                    if current_line.len() <= current_index {
                        break;
                    }

                    let last_index = if current_line.len() < current_index + max_len {
                        current_line.len()
                    } else {
                        current_index + max_len
                    };
                    let l = &current_line[current_index..];
                    match l.chars().position(|c| c == '\n') {
                        Some(i) => {
                            new_line += &l[0..i+1];
                            current_index += i + 1;
                        }
                        None => {
                            new_line += l;
                            if last_index != current_line.len() {
                                match l.chars().last() {
                                    Some('\n') => {},
                                    Some(_) => new_line += "\n",
                                    None => {},
                                }
                            }
                            current_index += max_len;
                        }
                    }
                }
                text.text = new_line;

                let lines = text.text.lines().count() as u16;
                let n = 4;
                if lines > n && lines > text.lines {
                    text.lines = lines;
                    text.current_scroll = lines - n - 1;
                }
            }
        }
        Mode::Exiting => match key.code {
            KeyCode::Char('y') => return true,
            KeyCode::Char('n') => {
                app.mode = Mode::Main;
            }
            _ => {}
        }
        _ => {}
    }

    return false;
}
