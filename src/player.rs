use std::collections::HashMap;

use anyhow::{Context, Result};
use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub struct MainPlayerData {
    pub username: String,
    pub online: bool,
    pub server: Option<String>,
    pub active_character: String,
    pub uuid: String,
    pub rank: Option<String>,
    pub rank_badge: Option<String>,
    pub legacy_rank_colour: Option<LegacyRankColour>,
    pub shortened_rank: Option<String>,
    pub support_rank: Option<String>,
    pub veteran: bool,
    pub first_join: String,
    pub last_join: String,
    pub playtime: f32,
    pub guild: Option<PlayerGuild>,
    pub global_data: GlobalData,
    pub forum_link: Option<i32>,
    pub ranking: HashMap<String, i32>,
    pub previous_ranking: HashMap<String, i32>,
    pub public_profile: bool,
}

#[derive(Debug)]
pub struct LegacyRankColour {
    pub main: String,
    pub sub: String,
}

#[derive(Debug)]
pub struct PlayerGuild {
    pub uuid: String,
    pub name: String,
    pub prefix: String,
    pub rank: String,
    pub rank_stars: String,
}

#[derive(Debug)]
pub struct GlobalData {
    pub wars: i32,
    pub total_level: i32,
    pub killed_mobs: i32,
    pub chests_found: i32,
    pub dungeons: DungeonData,
    pub raids: RaidData,
    pub completed_quests: i32,
    pub pvp: PvpData
}

#[derive(Debug)]
pub struct DungeonData {
    pub total: i32,
    pub dungeon_list: HashMap<String, i32>,
}

#[derive(Debug)]
pub struct RaidData {
    pub total: i32,
    pub raid_list: HashMap<String, i32>,
}

#[derive(Debug)]
pub struct PvpData {
    pub kills: i32,
    pub deaths: i32,
}

#[derive(Debug)]
pub struct OnlinePlayerData {
    pub total_online: i32,
    pub players_by_world: HashMap<String, Vec<String>>,
}

pub async fn get_player_main_stats(identifier: &str) -> Result<MainPlayerData> {
    let url = format!("https://api.wynncraft.com/v3/player/{}", identifier);

    let response = reqwest::get(&url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let player_object = data.as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected the JSON to be an object")
    })?;

    let username = player_object["username"].as_str().ok_or_else(|| {
        anyhow::Error::msg("Expected 'username' to be a String")
    })?.parse::<String>().unwrap();

    let online = player_object["online"].as_bool().ok_or_else(|| {
        anyhow::Error::msg("Expected 'online' to be a bool")
    })?;

    let server = player_object["server"].as_str().map(|s| s.to_string());

    let active_character = player_object["activeCharacter"].as_str().ok_or_else(|| {
        anyhow::Error::msg("Expected 'activeCharacter' to be a String")
    })?.parse::<String>().unwrap();

    let uuid = player_object["uuid"].as_str().ok_or_else(|| {
        anyhow::Error::msg("Expected 'uuid' to be a String")
    })?.parse::<String>().unwrap();

    let rank = player_object["rank"].as_str().map(|s| s.to_string());


    let rank_badge = player_object["rankBadge"].as_str().map(|s| s.to_string());

    let legacy_rank_colour = player_object["legacyRankColour"].as_object().map(|obj| {
        LegacyRankColour {
            main: obj["main"].as_str().unwrap().to_string(),
            sub: obj["sub"].as_str().unwrap().to_string(),
        }
    });

    let shortened_rank = player_object["shortenedRank"].as_str().map(|s| s.to_string());

    let support_rank = player_object["supportRank"].as_str().map(|s| s.to_string());

    let veteran = player_object["veteran"].as_bool().unwrap_or(false);

    let first_join = player_object["firstJoin"].as_str().ok_or_else(|| {
        anyhow::Error::msg("Expected 'firstJoin' to be a String")
    })?.parse::<String>().unwrap();

    let last_join = player_object["lastJoin"].as_str().ok_or_else(|| {
        anyhow::Error::msg("Expected 'lastJoin' to be a String")
    })?.parse::<String>().unwrap();

    let playtime = player_object["playtime"].as_f64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'playtime' to be a float")
    })? as f32;

    let guild = player_object["guild"].as_object().map(|obj| {
        PlayerGuild {
            uuid: obj["uuid"].as_str().unwrap().to_string(),
            name: obj["name"].as_str().unwrap().to_string(),
            prefix: obj["prefix"].as_str().unwrap().to_string(),
            rank: obj["rank"].as_str().unwrap().to_string(),
            rank_stars: obj["rankStars"].as_str().unwrap().to_string(),
        }
    });

    let global_data_object = player_object["globalData"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected the JSON to be an object")
    })?;

    let wars = global_data_object["wars"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'wars' to be an integer")
    })? as i32;

    let total_level = global_data_object["totalLevel"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'totalLevel' to be an integer")
    })? as i32;

    let killed_mobs = global_data_object["killedMobs"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'killedMobs' to be an integer")
    })? as i32;

    let chests_found = global_data_object["chestsFound"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'chestsFound' to be an integer")
    })? as i32;

    let dungeons_object = global_data_object["dungeons"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'dungeons' to be an object")
    })?;

    let dungeons = DungeonData {
        total: dungeons_object["total"].as_i64().ok_or_else(|| {
            anyhow::Error::msg("Expected 'total' to be an i64")
        })? as i32,
        dungeon_list: parse_list_to_hashmap(dungeons_object["list"].as_object().ok_or_else(|| {
            anyhow::Error::msg("Expected 'list' to be an object")
        })?)?,
    };

    let raids_object = global_data_object["raids"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'raids' to be an object")
    })?;

    let raids = RaidData {
        total: raids_object["total"].as_i64().ok_or_else(|| {
            anyhow::Error::msg("Expected 'total' to be an i64")
        })? as i32,
        raid_list: parse_list_to_hashmap(raids_object["list"].as_object().ok_or_else(|| {
            anyhow::Error::msg("Expected 'list' to be an object")
        })?)?,
    };

    let completed_quests = global_data_object["completedQuests"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'completedQuests' to be an integer")
    })? as i32;

    let pvp_object = global_data_object["pvp"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'pvp' to be an object")
    })?;

    let pvp = PvpData {
        kills: pvp_object["kills"].as_i64().ok_or_else(|| {
            anyhow::Error::msg("Expected 'kills' to be an i64")
        })? as i32,
        deaths: pvp_object["deaths"].as_i64().ok_or_else(|| {
            anyhow::Error::msg("Expected 'deaths' to be an i64")
        })? as i32,
    };

    let global_data = GlobalData {
        wars,
        total_level,
        killed_mobs,
        chests_found,
        dungeons,
        raids,
        completed_quests,
        pvp,
    };

    let forum_link = player_object["forumLink"].as_i64().map(|num| num as i32);

    let ranking_object = player_object["ranking"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'ranking' to be an object")
    })?;

    let ranking = parse_list_to_hashmap(ranking_object)?;

    let previous_ranking_object = player_object["previousRanking"].as_object().ok_or_else(|| {
        anyhow::Error::msg("Expected 'previousRanking' to be an object")
    })?;

    let previous_ranking = parse_list_to_hashmap(previous_ranking_object)?;

    let public_profile = player_object["publicProfile"].as_bool().ok_or_else(|| {
        anyhow::Error::msg("Expected 'publicProfile' to be a bool")
    })?;

    Ok(MainPlayerData {
        username,
        online,
        server,
        active_character,
        uuid,
        rank,
        rank_badge,
        legacy_rank_colour,
        shortened_rank,
        support_rank,
        veteran,
        first_join,
        last_join,
        playtime,
        guild,
        global_data,
        forum_link,
        ranking,
        previous_ranking,
        public_profile,
    })
}

async fn fetch_online_players(world: &str, uuid: bool) -> Result<Vec<String>> {
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
    fetch_online_players("", uuid).await
}

pub async fn get_online_players_on_world(world: i32, uuid: bool) -> Result<Vec<String>> {
    let world_str = format!("WC{}", world);
    fetch_online_players(&world_str, uuid).await
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

fn parse_list_to_hashmap(list_object: &serde_json::Map<String, Value>) -> Result<HashMap<String, i32>, anyhow::Error> {
    list_object.iter().map(|(key, value)| {
        Ok((key.clone(), value.as_i64().ok_or_else(|| {
            anyhow::Error::msg("Expected list values to be i64")
        })? as i32))
    }).collect()
}
