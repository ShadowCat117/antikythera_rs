#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    #[tokio::test]
    async fn test_get_online_players_username() {
        match get_online_players(false).await {
            Ok(player_list) => {
                println!("Player List Usernames: {:?}", player_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_players_uuid() {
        match get_online_players(true).await {
            Ok(player_list) => {
                println!("Player List UUIDs: {:?}", player_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_players_username_world() {
        match get_online_players_on_world(1, false).await {
            Ok(player_list) => {
                println!("Player List Usernames WC1: {:?}", player_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_players_uuid_world() {
        match get_online_players_on_world(1, true).await {
            Ok(player_list) => {
                println!("Player List UUIDs WC1: {:?}", player_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_players_count() {
        match get_online_player_count().await {
            Ok(player_count) => {
                println!("Online Player Count: {:?}", player_count);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_players_count_world() {
        match get_online_player_count_on_world(1).await {
            Ok(player_count) => {
                println!("Online Player Count WC1: {:?}", player_count);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_player_data_username() {
        match get_online_player_data(false).await {
            Ok(player_data) => {
                println!("Online Player Data Username: {:?}", player_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_online_player_data_uuid() {
        match get_online_player_data(true).await {
            Ok(player_data) => {
                println!("Online Player Data UUID: {:?}", player_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
