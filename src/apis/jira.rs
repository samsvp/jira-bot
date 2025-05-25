use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;
use anyhow::Result;

use super::requests;

#[derive(Deserialize)]
pub struct UserData {
    token: String,
    project: String,
    user: String,
}

impl UserData {
    pub fn url(&self, path: &str) -> String {
        format!("https://{}.atlassian.net/rest/api/3/{}", self.project, path)
    }
}

fn create_document(description: &str) -> serde_json::Value {
    json!({
        "content": [
            {
              "content": [
                {
                  "text": description,
                  "type": "text"
                }
              ],
              "type": "paragraph"
            }
          ],
        "type": "doc",
        "version": 1
    })
}

async fn get(
    user: &UserData,
    client: &reqwest::Client,
    url: &str,
) -> Result<serde_json::Value> {
    requests::get(
        client,
        &url,
        (&user.user, &user.token),
        HashMap::new(),
    ).await
}

pub async fn get_issues(
    user: &UserData,
    client: &reqwest::Client,
) -> Result<serde_json::Value> {
    let url = user.url("search");
    get(user, client, &url).await
}

pub async fn get_projects(
    user: &UserData,
    client: &reqwest::Client,
) -> Result<serde_json::Value> {
    let url = user.url("project");
    get(user, client, &url).await
}

pub async fn create_issue(
    user: &UserData,
    client: &reqwest::Client,
    summary: &str,
    issue_description: &str,
) -> Result<serde_json::Value> {
    let payload = json!({
        "fields": {
            "summary": summary,
            "project": {
                "id": "10000"
            },
            "issuetype": {
                "id": "10001"
            },
            "description": create_document(issue_description)
        }
    });
    let payload_str = serde_json::to_string(&payload).unwrap();
    let url = user.url("issue");
    requests::post(
        client,
        &url,
        Some((&user.user, &user.token)),
        HashMap::new(),
        payload_str,
    ).await
}

