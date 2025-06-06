pub mod apis;
pub mod interface;

use crate::apis::{jira,github};
use apis::openai;
use crossterm::event::{DisableMouseCapture, Event, KeyCode};
use crossterm::terminal::{self, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use crossterm::{event, terminal::enable_raw_mode};
use interface::app::{App, Mode};
use interface::ui;
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::{DefaultTerminal, Frame, Terminal};
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use std::io;
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Deserialize)]
struct Variables {
    github: github::UserData,
    jira: jira::UserData,
    openai: openai::UserData,
}

#[tokio::main]
async fn main() -> Result<()> {
    /**
    let issue = github::Issue {
        title: "Rust test".to_string(),
        body: "All cool.".to_string(),
    };

    let contents = fs::read_to_string("env.json")
        .expect("Should have been able to read the file");

    let vars: Variables = serde_json::from_str(&contents)?;
    let client = Client::new();

    let res = github::create_issue(&vars.github, &client, "jira-bot", issue).await;
    println!("{:?}", res);

    let res = jira::create_issue(&vars.jira, &client, "Test issue", "test description").await;
    println!("{:?}", res);

    let res = openai::create_story(&vars.openai, &client, "Crie uma story de usuário para: Tela de usuários (gerente): adicionar usuário").await;
    match res {
        Ok(r) =>
            println!("{}",  serde_json::to_string_pretty(&r).unwrap()),
        Err(e) =>
            println!("{:?}", e)
    };

    Ok(())
    */
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    loop {
        terminal.draw(|frame| ui::ui(frame, &app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.mode {
                Mode::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.mode = Mode::Exiting;
                    }
                    _ => {}
                },
                Mode::Exiting => match key.code {
                    KeyCode::Char('y') => break,
                    KeyCode::Char('n') => {
                        app.mode = Mode::Main;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
