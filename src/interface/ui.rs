use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use ratatui::Frame;

use super::app::{App, ChatText, Mode, Selected};

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

fn create_chat(
    chat: &ChatText,
    color: Color,
    running_section: Block,
    layout: Rect,
    frame: &mut Frame,
) {
    let p = Paragraph::new(
        Text::styled(
            &chat.text,
            Style::default().fg(color),
        ),
    )
        .block(running_section.clone())
        .wrap( Wrap { trim: true } )
        .scroll((chat.current_scroll, 0));

    let inner_area = running_section.inner(layout);
    let max_lines = inner_area.height as usize;
    frame.render_widget(p.scroll((chat.current_scroll, 0)), layout);

    let total_lines = chat.text.lines().count();
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        layout,
        &mut ScrollbarState::new(total_lines)
            .position(chat.current_scroll as usize)
            .viewport_content_length(max_lines),
    );
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
    match &app.mode {
        Mode::Main => {
            let next_mode = match &app.selected {
                Selected::Answer => "prompt",
                Selected::Prompt => "answer",
            };

            create_title("Scrum Master", Color::Green, create_section(None), layout[0], frame);
            create_title(
                &format!("Commands: [Tab] {next_mode} | [i]nsert | [s]end | [q]uit"),
                Color::Blue,
                create_section(None),
                layout[2],
                frame,
            );
        }
        Mode::Exiting => {
            let area = create_popup_layout(65, 10, frame.area());
            create_title("Quit? [y]es : [n]o", Color::Red, create_section(Some(Color::DarkGray)), area, frame);
        }
        Mode::Chat => {
            match app.selected {
                Selected::Prompt => {
                    create_title("Scrum Master - Insert Mode - Prompt", Color::Green, create_section(None), layout[0], frame);
                    create_title("Commands: [Esc] quit", Color::Blue, create_section(None), layout[2], frame);
                }
                Selected::Answer => {
                    create_title("Scrum Master - Insert Mode - Answer", Color::Green, create_section(None), layout[0], frame);
                    create_title("Commands: [Esc] quit", Color::Blue, create_section(None), layout[2], frame);
                }
            };
        }
        _ => {}
    };

    let p = 90;
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - p) / 2),
            Constraint::Percentage(p),
            Constraint::Percentage((100 - p) / 2),
        ])
        .split(layout[1])[1];

    let prompt_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(main_layout);

    let mut prompt_block = create_section(None);
    let mut answer_block = create_section(None);
    match app.selected {
        Selected::Prompt => prompt_block = prompt_block.fg(Color::LightYellow),
        Selected::Answer => answer_block = answer_block.fg(Color::LightYellow),
    }

    create_chat(&app.chat.prompt, Color::White, prompt_block, prompt_area[0], frame);
    create_chat(&app.chat.answer, Color::White, answer_block, prompt_area[1], frame);
}
