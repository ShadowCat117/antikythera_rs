#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    #[tokio::test]
    async fn test_get_latest_news() {
        match get_latest_news().await {
            Ok(latest_news) => {
                for news in latest_news.iter().take(3) {
                    println!("{:?}", news);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
