use anyhow::Context;
use serde_json::Value;

#[derive(Debug)]
pub struct NewsData {
    pub title: String,
    pub date: String,
    pub forum_thread: String,
    pub author: String,
    pub content: String,
    pub comments: String,
}

pub async fn get_latest_news() -> anyhow::Result<Vec<NewsData>> {
    let url = "https://api.wynncraft.com/v3/latest-news";

    let response = reqwest::get(url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let news_array = data.as_array().expect("Expected news data to be a JSON array");

    let mut news_data = Vec::new();
    for news_value in news_array {
        if let Some(news_object) = news_value.as_object() {
            let title = news_object.get("title").and_then(Value::as_str).unwrap_or_default().to_string();
            let date = news_object.get("date").and_then(Value::as_str).unwrap_or_default().to_string();
            let forum_thread = news_object.get("forumThread").and_then(Value::as_str).unwrap_or_default().to_string();
            let author = news_object.get("author").and_then(Value::as_str).unwrap_or_default().to_string();
            let content = news_object.get("content").and_then(Value::as_str).unwrap_or_default().to_string();
            let comments = news_object.get("comments").and_then(Value::as_str).unwrap_or_default().to_string();

            news_data.push(NewsData{ title, date, forum_thread, author, content, comments });
        }
    }

    Ok(news_data)
}