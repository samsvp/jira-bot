use reqwest::Client;
use std::error::Error;
use serde::{Serialize,Deserialize};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let issue = Issue {
        title: "Rust test".to_string(),
        body: "All cool.".to_string(),
    };

    let contents = fs::read_to_string("env.toml")
        .expect("Should have been able to read the file");

    let vars: GithubVariables = toml::from_str(&contents).unwrap();

    let url = format!("https://api.github.com/repos/{}/{}/issues", "samsvp", "jira-bot");
    let client = Client::new();
    let res = client
        .post(url)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", vars.token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "reqwest")
        .body(serde_json::to_string(&issue)?)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    println!("Status: {}", status);
    println!("Status: {}", body);

    Ok(())
}
