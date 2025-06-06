use crossterm::event::{self, KeyCode, KeyEvent};

use super::app::{App, Mode, Selected};

pub fn handle_input(key: &KeyEvent, app: &mut App) -> bool {
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
            KeyCode::Backspace =>  match &app.selected {
                Selected::Prompt => {
                    app.chat.prompt.pop();
                }
                Selected::Answer => {
                    app.chat.answer.pop();
                }
            }
            KeyCode::Char(c) => {
                match &app.selected {
                    Selected::Prompt => app.chat.prompt.push(c),
                    Selected::Answer => app.chat.answer.push(c),
                }
            }
            _ => {}
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
