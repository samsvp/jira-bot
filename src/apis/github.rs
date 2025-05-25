use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use anyhow::Result;

use super::requests;

#[derive(Serialize)]
pub struct Issue {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub token: String,
    pub username: String,
}

pub async fn create_issue(
    user: &UserData,
    client: &reqwest::Client,
    repo: &str,
    issue: Issue,
) -> Result<serde_json::Value> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        user.username,
        repo,
    );
    let headers = HashMap::from([
        ("Accept".to_string(), "application/vnd.github+json".to_string()),
        ("Authorization".to_string(), format!("Bearer {}", user.token)),
        ("X-GitHub-Api-Version".to_string(), "2022-11-28".to_string()),
        ("User-Agent".to_string(), "reqwest".to_string()),
    ]);
    let payload = serde_json::to_string(&issue)?;

    requests::post(
        client,
        &url,
        None,
        headers,
        payload
    ).await
}
