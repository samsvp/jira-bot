use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use super::app::{App, Mode};

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut create_title = |text, running_section, layout| {
        let title = Paragraph::new(
            Text::styled(
                text,
                Style::default().fg(Color::Green),
            ),
        ).block(running_section);
        frame.render_widget(title, layout);
    };

    // set footer and header
    match app.mode {
        Mode::Main => {
            create_title("Scrum Master", header_block, layout[0]);
        }
        _ => {}
    };
}
