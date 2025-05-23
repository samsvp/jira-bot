pub mod jira;
pub mod github;

use jira::jira::{Jira,JiraUserData};
use github::github::{Github, GithubUserData, Issue};
use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Deserialize)]
struct Variables {
    github: GithubUserData,
    jira: JiraUserData,
}

#[tokio::main]
async fn main() -> Result<()> {
    let issue = Issue {
        title: "Rust test".to_string(),
        body: "All cool.".to_string(),
    };

    let contents = fs::read_to_string("env.json")
        .expect("Should have been able to read the file");

    let vars: Variables = serde_json::from_str(&contents)?;


    let github_client = Github {
        user: vars.github,
        client: Client::new(),
    };
    let res = github_client.create_issue("jira-bot", issue).await;
    println!("{:?}", res);

    let jira_client = Jira {
        user: vars.jira,
        client: Client::new(),
    };

    let res = jira_client.create_issue("Test issue", "test description").await;
    println!("{:?}", res);

    Ok(())
}
