pub mod jira;

use jira::jira::{Jira,JiraUserData};
use reqwest::Client;
use serde::{Serialize,Deserialize};
use anyhow::Result;
use std::fs;


#[derive(Serialize)]
struct Issue {
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct GithubVariables {
    token: String,
}

#[derive(Deserialize)]
struct Variables {
    github: GithubVariables,
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

    let jira_client = Jira {
        user: vars.jira,
        client: Client::new(),
    };
    let response = jira_client.get_issues().await?;
    let v = jira_client.create_issue("Test issue".to_string(), "test description".to_string()).await;
    match v {
        Ok(res) => println!("POST: {}", res),
        Err(err) => println!("Error {}", err)
    }

    /**
    let url = format!("https://api.github.com/repos/{}/{}/issues", "samsvp", "jira-bot");
    let client = Client::new();
    let res = client
        .post(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", vars.github.token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "reqwest")
        .body(serde_json::to_string(&issue)?)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    println!("Status: {}", status);
    println!("Status: {}", body);
    */

    Ok(())
}
