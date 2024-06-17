use std::collections::HashMap;

use anyhow::{Context, Result};
use reqwest;
use serde_json::{Map, Value};

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
pub struct FullPlayerData {
    pub main_data: MainPlayerData,
    pub characters: HashMap<String, CharacterData>,
}

#[derive(Debug)]
pub struct CharacterData {
    pub class: String,
    pub nickname: Option<String>,
    pub level: i32,
    pub xp: i64,
    pub xp_percent: i32,
    pub total_level: i32,
    pub wars: i32,
    pub playtime: f32,
    pub mobs_killed: i32,
    pub chests_found: i32,
    pub items_identified: i32,
    pub blocks_walked: i64,
    pub logins: i32,
    pub deaths: i32,
    pub discoveries: i32,
    pub pre_economy: bool,
    pub pvp: PvpData,
    pub gamemodes: Vec<String>,
    pub skill_points: Option<SkillPointData>,
    pub professions: HashMap<String, ProfessionData>,
    pub dungeons: Option<DungeonData>,
    pub raids: Option<RaidData>,
    pub quests: Vec<String>,
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
    pub pvp: PvpData,
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
pub struct SkillPointData {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub agility: i32,
}

#[derive(Debug)]
pub struct ProfessionData {
    pub level: i32,
    pub xp_percent: i32,
}

#[derive(Debug)]
pub struct OnlinePlayerData {
    pub total_online: i32,
    pub players_by_world: HashMap<String, Vec<String>>,
}

async fn fetch_player_main_stats(data: &Value) -> Result<MainPlayerData> {
    let player_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let username = player_object["username"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'username' to be a String"))?
        .parse::<String>()
        .unwrap();

    let online = player_object["online"]
        .as_bool()
        .ok_or_else(|| anyhow::Error::msg("Expected 'online' to be a bool"))?;

    let server = player_object["server"].as_str().map(|s| s.to_string());

    let active_character = player_object["activeCharacter"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'activeCharacter' to be a String"))?
        .parse::<String>()
        .unwrap();

    let uuid = player_object["uuid"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'uuid' to be a String"))?
        .parse::<String>()
        .unwrap();

    let rank = player_object["rank"].as_str().map(|s| s.to_string());

    let rank_badge = player_object["rankBadge"].as_str().map(|s| s.to_string());

    let legacy_rank_colour =
        player_object["legacyRankColour"]
            .as_object()
            .map(|obj| LegacyRankColour {
                main: obj["main"].as_str().unwrap().to_string(),
                sub: obj["sub"].as_str().unwrap().to_string(),
            });

    let shortened_rank = player_object["shortenedRank"]
        .as_str()
        .map(|s| s.to_string());

    let support_rank = player_object["supportRank"].as_str().map(|s| s.to_string());

    let veteran = player_object["veteran"].as_bool().unwrap_or(false);

    let first_join = player_object["firstJoin"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'firstJoin' to be a String"))?
        .parse::<String>()
        .unwrap();

    let last_join = player_object["lastJoin"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'lastJoin' to be a String"))?
        .parse::<String>()
        .unwrap();

    let playtime = player_object["playtime"]
        .as_f64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'playtime' to be a float"))?
        as f32;

    let guild = player_object["guild"].as_object().map(|obj| PlayerGuild {
        uuid: obj["uuid"].as_str().unwrap().to_string(),
        name: obj["name"].as_str().unwrap().to_string(),
        prefix: obj["prefix"].as_str().unwrap().to_string(),
        rank: obj["rank"].as_str().unwrap().to_string(),
        rank_stars: obj["rankStars"].as_str().unwrap().to_string(),
    });

    let global_data_object = player_object["globalData"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let wars = global_data_object["wars"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'wars' to be an integer"))?
        as i32;

    let total_level = global_data_object["totalLevel"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'totalLevel' to be an integer"))?
        as i32;

    let killed_mobs = global_data_object["killedMobs"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'killedMobs' to be an integer"))?
        as i32;

    let chests_found = global_data_object["chestsFound"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'chestsFound' to be an integer"))?
        as i32;

    let dungeons = parse_dungeon_data(Some(
        global_data_object["dungeons"]
            .as_object()
            .ok_or_else(|| anyhow::Error::msg("Expected 'dungeons' to be an object"))?,
    ))?
    .ok_or_else(|| anyhow::Error::msg("Global dungeons data should not be null"))?;

    let raids =
        parse_raid_data(Some(global_data_object["raids"].as_object().ok_or_else(
            || anyhow::Error::msg("Expected 'raids' to be an object"),
        )?))?
        .ok_or_else(|| anyhow::Error::msg("Raids data should not be null"))?;

    let completed_quests = global_data_object["completedQuests"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'completedQuests' to be an integer"))?
        as i32;

    let pvp_object = global_data_object["pvp"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected 'pvp' to be an object"))?;

    let pvp = create_pvp_data(pvp_object)?;

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

    let ranking_object = player_object["ranking"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected 'ranking' to be an object"))?;

    let ranking = parse_list_to_hashmap(ranking_object)?;

    let previous_ranking_object = player_object["previousRanking"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected 'previousRanking' to be an object"))?;

    let previous_ranking = parse_list_to_hashmap(previous_ranking_object)?;

    let public_profile = player_object["publicProfile"]
        .as_bool()
        .ok_or_else(|| anyhow::Error::msg("Expected 'publicProfile' to be a bool"))?;

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

async fn fetch_character_data(data: &Value) -> Result<HashMap<String, CharacterData>> {
    let player_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let characters_object = player_object
        .get("characters")
        .ok_or_else(|| anyhow::Error::msg("Expected 'characters' to be a JSON object"))?
        .as_object()
        .unwrap();

    let mut characters: HashMap<String, CharacterData> = HashMap::new();

    for (character, character_details) in characters_object {
        let class = character_details["type"]
            .as_str()
            .ok_or_else(|| anyhow::Error::msg("Expected 'type' to be a String"))?
            .parse::<String>()
            .unwrap();

        let nickname = character_details["nickname"]
            .as_str()
            .map(|s| s.to_string());

        let level = character_details["level"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'level' to be an integer"))?
            as i32;

        let xp = character_details["xp"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'xp' to be an integer"))?;

        let xp_percent = character_details["xpPercent"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'xpPercent' to be an integer"))?
            as i32;

        let total_level = character_details["totalLevel"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'totalLevel' to be an integer"))?
            as i32;

        let wars = character_details["wars"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'wars' to be an integer"))?
            as i32;

        let playtime = character_details["playtime"]
            .as_f64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'playtime' to be an integer"))?
            as f32;

        let mobs_killed = character_details["mobsKilled"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'mobsKilled' to be an integer"))?
            as i32;

        let chests_found = character_details["chestsFound"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'chestsFound' to be an integer"))?
            as i32;

        let items_identified = character_details["itemsIdentified"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'itemsIdentified' to be an integer"))?
            as i32;

        let blocks_walked = character_details["blocksWalked"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'blocksWalked' to be an integer"))?;

        let logins = character_details["logins"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'logins' to be an integer"))?
            as i32;

        let deaths = character_details["deaths"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'deaths' to be an integer"))?
            as i32;

        let discoveries = character_details["discoveries"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'discoveries' to be an integer"))?
            as i32;

        let pre_economy = character_details["preEconomy"]
            .as_bool()
            .ok_or_else(|| anyhow::Error::msg("Expected 'preEconomy' to be a bool"))?;

        let pvp_object = character_details["pvp"]
            .as_object()
            .ok_or_else(|| anyhow::Error::msg("Expected 'pvp' to be an object"))?;

        let pvp = create_pvp_data(pvp_object)?;

        let gamemode_array = character_details["gamemode"]
            .as_array()
            .expect("Expected gamemode data to be a JSON array");

        let gamemodes: Vec<String> = gamemode_array
            .iter()
            .map(|gamemode_value| {
                gamemode_value
                    .as_str()
                    .expect("Expected gamemode value to be a string")
                    .to_string()
            })
            .collect();

        let skill_points = character_details["skillPoints"]
            .as_object()
            .and_then(|obj| {
                if obj.is_empty() {
                    None
                } else {
                    Some(SkillPointData {
                        strength: obj.get("strength").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                        dexterity: obj.get("dexterity").and_then(|v| v.as_i64()).unwrap_or(0)
                            as i32,
                        intelligence: obj
                            .get("intelligence")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0) as i32,
                        defense: obj.get("defense").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                        agility: obj.get("agility").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                    })
                }
            });

        let professions_obj = character_details["professions"]
            .as_object()
            .expect("Expected 'professions' to be an object");

        let mut professions: HashMap<String, ProfessionData> = HashMap::new();

        for (profession, data) in professions_obj {
            let profession_data = ProfessionData {
                level: data["level"].as_i64().unwrap_or(1) as i32,
                xp_percent: data["xpPercent"].as_i64().unwrap_or(0) as i32,
            };
            professions.insert(profession.to_string(), profession_data);
        }

        let dungeons = parse_dungeon_data(character_details["dungeons"].as_object())?;

        let raids = parse_raid_data(character_details["raids"].as_object())?;

        let quests_array = character_details["quests"]
            .as_array()
            .expect("Expected quests data to be a JSON array");

        let quests: Vec<String> = quests_array
            .iter()
            .map(|quest_value| {
                quest_value
                    .as_str()
                    .expect("Expected quest value to be a string")
                    .to_string()
            })
            .collect();

        characters.insert(
            character.to_string(),
            CharacterData {
                class,
                nickname,
                level,
                xp,
                xp_percent,
                total_level,
                wars,
                playtime,
                mobs_killed,
                chests_found,
                items_identified,
                blocks_walked,
                logins,
                deaths,
                discoveries,
                pre_economy,
                pvp,
                gamemodes,
                skill_points,
                professions,
                dungeons,
                raids,
                quests,
            },
        );
    }

    Ok(characters)
}

pub async fn get_player_main_stats(identifier: &str) -> Result<MainPlayerData> {
    let url = format!("https://api.wynncraft.com/v3/player/{}", identifier);

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    fetch_player_main_stats(&data).await
}

pub async fn get_player_full_stats(identifier: &str) -> Result<FullPlayerData> {
    let url = format!(
        "https://api.wynncraft.com/v3/player/{}?fullResult=True",
        identifier
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    Ok(FullPlayerData {
        main_data: fetch_player_main_stats(&data).await?,
        characters: fetch_character_data(&data).await?,
    })
}

async fn fetch_online_players(world: &str, uuid: bool) -> Result<Vec<String>> {
    let identifier = if uuid { "uuid" } else { "username" };
    let world_query = if !world.is_empty() {
        format!("&server={}", world)
    } else {
        String::new()
    };
    let url = format!(
        "https://api.wynncraft.com/v3/player?identifier={}{world_query}",
        identifier
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let players = data["players"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected 'players' to be a JSON object"))?;

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
    let world_query = if !world.is_empty() {
        format!("?identifier=username&server={}", world)
    } else {
        String::new()
    };
    let url = format!("https://api.wynncraft.com/v3/player{world_query}");

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let total_players = data["total"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'total' to be an integer"))?;

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
    let url = format!(
        "https://api.wynncraft.com/v3/player?identifier={}",
        identifier
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let total_players = data["total"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'total' to be an integer"))?
        as i32;

    let players_obj = data["players"]
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected 'players' to be a JSON object"))?;

    let mut players_by_world: HashMap<String, Vec<String>> = HashMap::new();
    for (player, world) in players_obj {
        players_by_world
            .entry(world.as_str().unwrap_or_default().to_string())
            .or_insert_with(Vec::new)
            .push(player.clone());
    }

    Ok(OnlinePlayerData {
        players_by_world,
        total_online: total_players,
    })
}

fn parse_list_to_hashmap(
    list_object: &Map<String, Value>,
) -> Result<HashMap<String, i32>, anyhow::Error> {
    list_object
        .iter()
        .map(|(key, value)| {
            Ok((
                key.clone(),
                value
                    .as_i64()
                    .ok_or_else(|| anyhow::Error::msg("Expected list values to be i64"))?
                    as i32,
            ))
        })
        .collect()
}

fn create_pvp_data(pvp_object: &Map<String, Value>) -> Result<PvpData, anyhow::Error> {
    Ok(PvpData {
        kills: pvp_object["kills"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'kills' to be an i64"))?
            as i32,
        deaths: pvp_object["deaths"]
            .as_i64()
            .ok_or_else(|| anyhow::Error::msg("Expected 'deaths' to be an i64"))?
            as i32,
    })
}

fn parse_dungeon_data(
    dungeons_object: Option<&Map<String, Value>>,
) -> Result<Option<DungeonData>, anyhow::Error> {
    match dungeons_object {
        Some(dungeons) => {
            let total = dungeons["total"]
                .as_i64()
                .ok_or_else(|| anyhow::Error::msg("Expected 'total' to be an i64"))?
                as i32;
            let dungeon_list = parse_list_to_hashmap(
                dungeons["list"]
                    .as_object()
                    .ok_or_else(|| anyhow::Error::msg("Expected 'list' to be an object"))?,
            )?;
            Ok(Some(DungeonData {
                total,
                dungeon_list,
            }))
        }
        None => Ok(None),
    }
}

fn parse_raid_data(
    raids_object: Option<&Map<String, Value>>,
) -> Result<Option<RaidData>, anyhow::Error> {
    match raids_object {
        Some(raids) => {
            let total = raids["total"]
                .as_i64()
                .ok_or_else(|| anyhow::Error::msg("Expected 'total' to be an i64"))?
                as i32;
            let raid_list = parse_list_to_hashmap(
                raids["list"]
                    .as_object()
                    .ok_or_else(|| anyhow::Error::msg("Expected 'list' to be an object"))?,
            )?;
            Ok(Some(RaidData { total, raid_list }))
        }
        None => Ok(None),
    }
}
