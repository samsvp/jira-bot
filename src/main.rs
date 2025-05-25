pub mod apis;

use crate::apis::{jira,github};
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Deserialize)]
struct Variables {
    github: github::UserData,
    jira: jira::UserData,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    Ok(())
}
