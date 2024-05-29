#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    #[tokio::test]
    async fn test_get_map_markers() {
        match get_map_markers().await {
            Ok(map_markers) => {
                for marker in map_markers.iter().take(5) {
                    println!("{:?}", marker);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_quest_count() {
        match get_quest_count().await {
            Ok(quest_count) => {
                assert_eq!(quest_count, 262, "The current number of quests is 262");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}