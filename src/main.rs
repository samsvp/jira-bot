pub mod apis;
pub mod interface;

use crate::apis::{jira,github};
use apis::openai;
use crossterm::event;
use interface::app::App;
use ratatui::{DefaultTerminal, Frame};
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

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, event::Event::Key(_)) {
            break Ok(());
        }
    }
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
    Ok(())
}
