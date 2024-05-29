#[cfg(test)]
mod tests {
    use antikythera_rs::*;

    #[tokio::test]
    async fn test_get_guilds_username() {
        match get_guilds(false).await {
            Ok(guilds_list) => {
                println!("Guild List Names: {:?}", guilds_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guilds_uuid() {
        match get_guilds(true).await {
            Ok(guilds_list) => {
                println!("Guild List UUIDs: {:?}", guilds_list);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guilds_data() {
        match get_guilds_data().await {
            Ok(guilds_data) => {
                for guild in guilds_data.iter().take(5) {
                    println!("{:?}", guild);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_territory_data() {
        match get_territory_data().await {
            Ok(territory_data) => {
                for territory in territory_data.iter().take(3) {
                    println!("{:?}", territory);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guild_from_name_username() {
        match get_guild_from_name("Chiefs Of Corkus", false).await {
            Ok(guild_data) => {
                println!("Guild Data: {:?}", guild_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guild_from_name_uuid() {
        match get_guild_from_name("Chiefs Of Corkus", true).await {
            Ok(guild_data) => {
                println!("Guild Data: {:?}", guild_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guild_from_prefix_username() {
        match get_guild_from_prefix("HOC", false).await {
            Ok(guild_data) => {
                println!("Guild Data: {:?}", guild_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_guild_from_prefix_uuid() {
        match get_guild_from_prefix("HOC", true).await {
            Ok(guild_data) => {
                println!("Guild Data: {:?}", guild_data);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
