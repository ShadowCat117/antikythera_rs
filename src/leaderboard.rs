use anyhow::Context;
use serde_json::Value;

#[derive(Debug)]
pub struct Leaderboard {
    pub position: i32,
    pub name: String,
    pub uuid: String,
    pub score: i32,
    pub previous_ranking: i32,
    // metadata, check all lb types for what is here
    pub nickname: String,
    pub character_uuid: String,
    pub character_type: String,
    pub rank: String,
    pub rank_badge: String,
    pub support_rank: String,
    // legacy_rank_colour special type
}

// woodcuttingLevel: xp, playtime
// miningLevel: xp, playtime
// fishingLevel: xp, playtime
// farmingLevel: xp, playtime
// alchemismLevel: xp, playtime
// armouringLevel: xp, playtime
// cookingLevel: xp, playtime
// jewelingLevel: xp, playtime
// scribingLevel: xp, playtime
// tailoringLevel: xp, playtime
// weaponsmithingLevel: xp, playtime
// woodworkingLevel: xp, playtime
// professionsGlobalLevel: xp, playtime
// combatGlobalLevel: xp, playtime
// totalGlobalLevel: xp, playtime
// playerContent: totalLevel, xp, playtime
// combatSoloLevel: totalLevel, xp, playtime
// professionsSoloLevel: totalLevel, xp, playtime
// totalSoloLevel: totalLevel, xp, playtime
// globalPlayerContent: playtime
// hardcoreLegacyLevel: totalLevel, xp, playtime also an extra characterData field
// nogCompletion: playtime
// tccCompletion: playtime
// nolCompletion: playtime
// warsCompletion: playtime
// ironmanContent: totalLevel, xp, playtime
// tnaCompletion: playtime
// ultimateIronmanContent: totalLevel, xp, playtime
// hardcoreContent: totalLevel, xp, playtime also an extra characterData field
// craftsmanContent: totalLevel, xp, playtime
// huntedContent: xp, playtime
// huicContent: totalLevel, xp, playtime also an extra characterData field
// huichContent: totalLevel, xp, playtime also an extra characterData field
// hichContent: totalLevel, xp, playtime also an extra characterData field
// hicContent: totalLevel, xp, playtime also an extra characterData field

// async fn fetch_leaderboard(lb_type: &str, limit: i32) -> anyhow::Result<Vec<Leaderboard>> {
//     let url = format!("https://api.wynncraft.com/v3/leaderboards/{}?resultLimit={}", lb_type, limit);
//
//     let response = reqwest::get(url).await.context("Failed to make the API request")?;
//
//     let data: Value = response.json().await.context("Failed to parse the JSON response")?;
//
//
// }

// pub async fn get_leaderboard(lb_type: &str) -> anyhow::Result<Vec<Leaderboard>> {
//     fetch_leaderboard(lb_type, 100).await
// }

// pub async fn get_leaderboard_with_limit(lb_type: &str, limit: i32) -> anyhow::Result<Vec<Leaderboard>> {
//     fetch_leaderboard(lb_type, limit).await
// }

pub async fn get_leaderboard_types() -> anyhow::Result<Vec<String>> {
    let url = "https://api.wynncraft.com/v3/leaderboards/types";

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let lb_types_array = data
        .as_array()
        .expect("Expected marker data to be a JSON array");

    let mut lb_types = Vec::new();
    for lb_type_value in lb_types_array {
        if let Some(lb_type) = lb_type_value.as_str() {
            lb_types.push(lb_type.to_string());
        }
    }

    Ok(lb_types)
}
