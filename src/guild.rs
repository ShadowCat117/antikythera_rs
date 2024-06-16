use crate::{json_to_location, Location};
use anyhow::Context;
use serde_json::Value;

#[derive(Debug)]
pub struct GuildData {
    pub uuid: String,
    pub name: String,
    pub prefix: String,
    pub level: i32,
    pub xp_percent: i32,
    pub territories: i32,
    pub wars: i32,
    pub created: String,
    pub members: Vec<GuildMember>,
    pub online: i32,
    pub banner: GuildBanner,
    pub season_ranks: Vec<SeasonRank>,
}

#[derive(Debug)]
pub struct GuildMember {
    pub username: String,
    pub uuid: String,
    pub online: bool,
    pub server: String,
    pub rank: String,
    pub contributed: i64,
    pub contribution_rank: i32,
    pub joined: String,
}

#[derive(Debug)]
pub struct SeasonRank {
    pub season: i32,
    pub rating: i32,
    pub final_territories: i32,
}

#[derive(Debug)]
pub struct GuildBanner {
    pub base: String,
    pub tier: i32,
    pub structure: String,
    pub layers: Vec<BannerLayer>,
}

#[derive(Debug)]
pub struct BannerLayer {
    pub colour: String,
    pub pattern: String,
}

#[derive(Debug)]
pub struct SimpleGuildData {
    pub uuid: String,
    pub name: String,
    pub prefix: String,
}

#[derive(Debug)]
pub struct TerritoryData {
    pub territory_name: String,
    pub owner: SimpleGuildData,
    pub acquired: String,
    pub start_location: Location,
    pub end_location: Location,
}

async fn fetch_guilds(uuid: bool) -> anyhow::Result<Vec<String>> {
    let identifier = if uuid { "uuid" } else { "name" };
    let url = format!(
        "https://api.wynncraft.com/v3/guild/list/guild?identifier={}",
        identifier
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let guild_data = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected data to be a JSON object"))?;

    let guilds: Vec<String> = guild_data.keys().cloned().collect();

    Ok(guilds)
}

pub async fn get_guilds(uuid: bool) -> anyhow::Result<Vec<String>> {
    fetch_guilds(uuid).await
}

pub async fn get_guilds_data() -> anyhow::Result<Vec<SimpleGuildData>> {
    let url = "https://api.wynncraft.com/v3/guild/list/guild?identifier=uuid";

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let guilds_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let mut guilds_data = Vec::new();
    for (uuid, details) in guilds_object {
        if let Some(guild_details) = details.as_object() {
            let name = guild_details
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let prefix = guild_details
                .get("prefix")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            guilds_data.push(SimpleGuildData {
                uuid: uuid.clone(),
                name,
                prefix,
            });
        }
    }

    Ok(guilds_data)
}

fn fetch_guild(data: Value, uuid: bool) -> anyhow::Result<GuildData> {
    let guild_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let guild_uuid = guild_object["uuid"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'uuid' to be a String"))?
        .parse::<String>()
        .unwrap();

    let name = guild_object["name"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'name' to be a String"))?
        .parse::<String>()
        .unwrap();

    let prefix = guild_object["prefix"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'prefix' to be a String"))?
        .parse::<String>()
        .unwrap();

    let level = guild_object["level"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'level' to be an integer"))?
        as i32;

    let xp_percent = guild_object["xpPercent"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'xpPercent' to be an integer"))?
        as i32;

    let territories = guild_object["territories"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'territories' to be an integer"))?
        as i32;

    let wars = guild_object["wars"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'wars' to be an integer"))?
        as i32;

    let created = guild_object["created"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'created' to be a String"))?
        .parse::<String>()
        .unwrap();

    let online = guild_object["online"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'online' to be an integer"))?
        as i32;

    let mut members = Vec::new();

    let members_object = guild_object
        .get("members")
        .ok_or_else(|| anyhow::Error::msg("Expected 'members' to be a JSON object"))?
        .as_object()
        .unwrap();

    for (rank, member_details) in members_object {
        if rank == "total" {
            continue;
        }

        for (member_identifier, details) in member_details.as_object().unwrap() {
            let member_uuid;
            let username;

            if uuid {
                member_uuid = member_identifier.to_string();
                username = details
                    .get("username")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
            } else {
                member_uuid = details
                    .get("uuid")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                username = member_identifier.to_string();
            }

            let online = details
                .get("online")
                .and_then(Value::as_bool)
                .unwrap_or_default();
            let server = details
                .get("server")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let contributed = details
                .get("contributed")
                .and_then(Value::as_i64)
                .unwrap_or_default();
            let contribution_rank = details
                .get("contributionRank")
                .and_then(Value::as_i64)
                .unwrap_or_default() as i32;
            let joined = details
                .get("joined")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();

            members.push(GuildMember {
                username: username.to_string(),
                uuid: member_uuid,
                online,
                server,
                rank: rank.to_string(),
                contributed,
                contribution_rank,
                joined,
            });
        }
    }

    let banner_object = guild_object
        .get("banner")
        .ok_or_else(|| anyhow::Error::msg("Expected 'banner' to be a JSON object"))?
        .as_object()
        .unwrap();

    let base = banner_object["base"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'base' to be a String"))?
        .parse::<String>()
        .unwrap();

    let tier = banner_object["tier"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'tier' to be an integer"))?
        as i32;

    let structure = banner_object["structure"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'structure' to be a String"))?
        .parse::<String>()
        .unwrap();

    let layers_array = banner_object["layers"]
        .as_array()
        .expect("Expected 'layers' to be a JSON array");
    let mut layers = Vec::new();

    for layer_value in layers_array {
        let colour = layer_value["colour"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let pattern = layer_value["pattern"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        layers.push(BannerLayer { colour, pattern });
    }

    let banner = GuildBanner {
        base,
        tier,
        structure,
        layers,
    };

    let season_ranks_object = guild_object["seasonRanks"]
        .as_object()
        .expect("Expected 'seasonRanks' to be a JSON object");

    let mut season_ranks = Vec::new();

    for (season, details) in season_ranks_object {
        let season_number = season
            .parse::<i32>()
            .expect("Expected season number to be an integer");
        let rating = details["rating"]
            .as_i64()
            .expect("Expected 'rating' to be an integer") as i32;
        let final_territories = details["finalTerritories"]
            .as_i64()
            .expect("Expected 'finalTerritories' to be an integer")
            as i32;

        season_ranks.push(SeasonRank {
            season: season_number,
            rating,
            final_territories,
        });
    }

    Ok(GuildData {
        uuid: guild_uuid,
        name,
        prefix,
        level,
        xp_percent,
        territories,
        wars,
        created,
        members,
        online,
        banner,
        season_ranks,
    })
}

async fn fetch_guild_name(name: &str, uuid: bool) -> anyhow::Result<GuildData> {
    let identifier = if uuid { "uuid" } else { "username" };
    let url = format!(
        "https://api.wynncraft.com/v3/guild/{}?identifier={}",
        name, identifier
    );

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    fetch_guild(data, uuid)
}

pub async fn get_guild_from_name(name: &str, uuid: bool) -> anyhow::Result<GuildData> {
    fetch_guild_name(name, uuid).await
}

async fn fetch_guild_prefix(prefix: &str, uuid: bool) -> anyhow::Result<GuildData> {
    let identifier = if uuid { "uuid" } else { "username" };
    let url = format!(
        "https://api.wynncraft.com/v3/guild/prefix/{}?identifier={}",
        prefix, identifier
    );

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    fetch_guild(data, uuid)
}

pub async fn get_guild_from_prefix(prefix: &str, uuid: bool) -> anyhow::Result<GuildData> {
    fetch_guild_prefix(prefix, uuid).await
}

pub async fn get_territory_data() -> anyhow::Result<Vec<TerritoryData>> {
    let url = "https://api.wynncraft.com/v3/guild/list/territory";

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let territories_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let mut territory_data = Vec::new();
    for (territory_name, details) in territories_object {
        if let Some(territory_details) = details.as_object() {
            let owner = territory_details
                .get("guild")
                .and_then(Value::as_object)
                .ok_or_else(|| anyhow::Error::msg("Expected 'guild' to be a JSON object"))?;
            let simple_guild_data = SimpleGuildData {
                uuid: owner
                    .get("uuid")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                name: owner
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                prefix: owner
                    .get("prefix")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            };
            let acquired = territory_details
                .get("acquired")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let location = territory_details
                .get("location")
                .and_then(Value::as_object)
                .ok_or_else(|| anyhow::Error::msg("Expected 'location' to be a JSON object"))?;
            let start_location = json_to_location(location.get("start").unwrap());
            let end_location = json_to_location(location.get("end").unwrap());

            territory_data.push(TerritoryData {
                territory_name: territory_name.clone(),
                owner: simple_guild_data,
                acquired,
                start_location,
                end_location,
            });
        }
    }

    Ok(territory_data)
}
