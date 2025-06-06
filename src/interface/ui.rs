use ratatui::layout::{self, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use super::app::{App, Mode};

fn create_popup_layout(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn create_title(
    text: &str,
    color: Color,
    running_section: Block,
    layout: Rect,
    frame: &mut Frame,
) {
        let title = Paragraph::new(
            Text::styled(
                text,
                Style::default().fg(color),
            ),
        )
            .block(running_section)
            .centered();

        frame.render_widget(title, layout);
}

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let create_section = |color| {
        let b = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        match color {
            Some(color) => b.bg(color),
            None => b,
        }
    };

    // set footer and header
    match app.mode {
        Mode::Main => {
            create_title("Scrum Master", Color::Green, create_section(None), layout[0], frame);
            create_title("Press q to quit", Color::Blue, create_section(None), layout[2], frame);
        }
        Mode::Exiting => {
            let area = create_popup_layout(65, 10, frame.area());
            create_title("Quit? [y]es : [n]o", Color::Red, create_section(Some(Color::DarkGray)), area, frame);
        }
        _ => {}
    };
}
