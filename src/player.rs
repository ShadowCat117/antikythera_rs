use std::collections::HashMap;

use anyhow::{Context, Result};
use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub struct OnlinePlayerData {
    pub total_online: i32,
    pub players_by_world: HashMap<String, Vec<String>>,
}

async fn fetch_players(world: &str, uuid: bool) -> Result<Vec<String>> {
    let identifier = if uuid { "uuid" } else { "username" };
    let world_query = if !world.is_empty() { format!("&server={}", world) } else { String::new() };
    let url = format!("https://api.wynncraft.com/v3/player?identifier={}{world_query}", identifier);

    let response = reqwest::get(&url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let players = data["players"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'players' to be a JSON object")
    })?;

    let player_names: Vec<String> = players.keys().cloned().collect();

    Ok(player_names)
}

pub async fn get_online_players(uuid: bool) -> Result<Vec<String>> {
    fetch_players("", uuid).await
}

pub async fn get_online_players_on_world(world: i32, uuid: bool) -> Result<Vec<String>> {
    let world_str = format!("WC{}", world);
    fetch_players(&world_str, uuid).await
}

async fn fetch_player_count(world: &str) -> Result<i32> {
    let world_query = if !world.is_empty() { format!("?identifier=username&server={}", world) } else { String::new() };
    let url = format!("https://api.wynncraft.com/v3/player{world_query}");

    let response = reqwest::get(&url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let total_players = data["total"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'total' to be an integer")
    })?;

    Ok(total_players as i32)
}

pub async fn get_online_player_count() -> Result<i32> {
    fetch_player_count("").await
}

pub async fn get_online_player_count_on_world(world: i32) -> Result<i32> {
    let world_str = format!("WC{}", world);
    fetch_player_count(&world_str).await
}

pub async fn get_online_player_data(uuid: bool) -> Result<OnlinePlayerData> {
    let identifier = if uuid { "uuid" } else { "username" };
    let url = format!("https://api.wynncraft.com/v3/player?identifier={}", identifier);

    let response = reqwest::get(&url).await.context("Failed to make the API request")?;
    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let total_players = data["total"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'total' to be an integer")
    })? as i32;

    let players_obj = data["players"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'players' to be a JSON object")
    })?;

    let mut players_by_world: HashMap<String, Vec<String>> = HashMap::new();
    for (player, world) in players_obj {
        players_by_world.entry(world.as_str().unwrap_or_default().to_string())
            .or_insert_with(Vec::new)
            .push(player.clone());
    }

    Ok(OnlinePlayerData {
        players_by_world,
        total_online: total_players,
    })
}
