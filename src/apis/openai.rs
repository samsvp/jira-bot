use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;
use anyhow::Result;

use super::requests::post;

#[derive(Deserialize)]
pub enum Lang {
    EN,
    PT,
}

#[derive(Deserialize)]
pub struct UserData {
    token: String,
    lang: Lang,
    model: String,
    base_url: String,
    context: Option<String>,
}

fn create_completion(model: &str, content: &str, context: String) -> String {
    let completion = json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": context
            },
            {
                "role": "user",
                "content": content
            }
        ]
    });
    serde_json::to_string(&completion).unwrap()
}

pub async fn create_story(
    data: &UserData,
    client: &reqwest::Client,
    content: &str,
) -> Result<serde_json::Value> {
    let context = match &data.context {
        Some(context) => context.to_owned(),
        None => match data.lang {
            Lang::EN => "You're a scrum master creating user stories.".to_string(),
            Lang::PT => "Você é um scrum master criando stories de usuário.".to_string()
        }
    };

    let headers = HashMap::from([
        ("Authorization".to_string(), format!("Bearer {}", data.token)),
    ]);

    let payload = create_completion(&data.model, content, context);
    let url = format!("{}/chat/completions", data.base_url);
    post(client, &url, None, headers, payload).await
}
