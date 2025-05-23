use serde::{Serialize,Deserialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Serialize)]
pub struct Issue {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct GithubUserData {
    pub token: String,
    pub username: String,
}

pub struct Github {
    pub user: GithubUserData,
    pub client: Client,
}

impl Github {
    pub async fn create_issue(
        &self,
        repo: &str,
        issue: Issue,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.user.username,
            repo,
        );
        let body = self.client
            .post(url)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.user.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "reqwest")
            .body(serde_json::to_string(&issue)?)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&body)?)
    }
}
