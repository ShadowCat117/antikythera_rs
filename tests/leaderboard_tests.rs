#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    // #[tokio::test]
    // async fn test_get_leaderboard() {
    //     match get_leaderboard_types().await {
    //         Ok(leaderboard_types) => {
    //             println!("Leaderboard Types: {:?}", leaderboard_types);
    //         }
    //         Err(e) => {
    //             println!("Error: {:?}", e);
    //         }
    //     }
    // }

    #[tokio::test]
    async fn test_get_leaderboard_types() {
        match get_leaderboard_types().await {
            Ok(leaderboard_types) => {
                println!("Leaderboard Types: {:?}", leaderboard_types);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}