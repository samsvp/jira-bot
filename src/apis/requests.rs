use std::collections::HashMap;
use anyhow::Result;

pub async fn get (
    client: &reqwest::Client,
    url: &str,
    auth: (&str, &str),
    headers: HashMap<String, String>,
) -> Result<serde_json::Value> {
    let body =
        headers.iter().fold(
            client.post(url),
            |req, item| {
                req.header(item.0, item.1)
            }
        )
        .header("Accept", "application/json")
        .basic_auth(auth.0, Some(auth.1))
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&body)?)
}

pub async fn post (
    client: &reqwest::Client,
    url: &str,
    auth: Option<(&str, &str)>,
    headers: HashMap<String, String>,
    payload: String,
) -> Result<serde_json::Value> {
    let req =
        headers.iter().fold(
            client.post(url),
            |req, item| {
                req.header(item.0, item.1)
            }
        )
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(payload);

    let body =
        match auth {
            Some(auth) =>
                req.basic_auth(auth.0, Some(auth.1)),
            None =>
                req
        }
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&body)?)
}
