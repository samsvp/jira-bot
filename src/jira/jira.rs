use serde::Deserialize;
use serde_json::json;
use reqwest::{Client, RequestBuilder};
use anyhow::Result;

#[derive(Deserialize)]
pub struct JiraUserData {
    token: String,
    project: String,
    user: String,
}

impl JiraUserData {
    pub fn url(&self, path: String) -> String {
        format!("https://{}.atlassian.net/rest/api/3/{}", self.project, path)
    }
}

pub struct Jira {
    pub user: JiraUserData,
    pub client: Client,
}

impl Jira {
    fn create_document(description: String) -> serde_json::Value {
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

    fn add_headers(&self, builder: RequestBuilder) -> RequestBuilder {
        builder
            .header("Accept", "application/json")
            .basic_auth(&self.user.user, Some(&self.user.token))
    }

    async fn make_post_request(&self, url: String, payload: String) -> Result<serde_json::Value> {
        let body =
            self.add_headers(self.client.post(url))
                .header("Content-Type", "application/json")
                .body(payload)
                .send()
                .await?
                .text()
                .await?;
        println!("Body is {}", body);
        Ok(serde_json::from_str(&body)?)
    }

    async fn make_get_request(&self, url: String) -> Result<serde_json::Value> {
        let body =
            self.add_headers(self.client.get(url))
                .send()
                .await?
                .text()
                .await?;
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_issues(&self) -> Result<serde_json::Value> {
        let url = self.user.url("search".to_string());
        self.make_get_request(url).await
    }

    pub async fn get_projects(&self) -> Result<serde_json::Value> {
        let url = self.user.url("project".to_string());
        self.make_get_request(url).await
    }

    pub async fn create_issue(
        &self,
        summary: String,
        issue_description: String,
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
                "description": Self::create_document(issue_description)
            }
        });
        let payload_str = serde_json::to_string(&payload).unwrap();
        let url = self.user.url("issue".to_string());
        self.make_post_request(url, payload_str).await
    }
}

