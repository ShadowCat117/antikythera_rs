use anyhow::Context;
use serde_json::Value;
use crate::Location;

#[derive(Debug)]
pub struct Marker {
    pub name: String,
    pub icon: String,
    pub location: Location,
}

pub async fn get_map_markers() -> anyhow::Result<Vec<Marker>> {
    let url = "https://api.wynncraft.com/v3/map/locations/markers";

    let response = reqwest::get(url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let markers_array = data.as_array().expect("Expected marker data to be a JSON array");

    let mut markers_data = Vec::new();
    for marker_value in markers_array {
        if let Some(marker_object) = marker_value.as_object() {
            let name = marker_object.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
            let icon = marker_object.get("icon").and_then(Value::as_str).unwrap_or_default().to_string();
            let x = marker_object["x"].as_str().unwrap_or("0").parse::<i32>().context("Failed to parse 'x' coordinate to i32")?;
            let y = marker_object["y"].as_str().unwrap_or("0").parse::<i32>().context("Failed to parse 'y' coordinate to i32")?;
            let z = marker_object["z"].as_str().unwrap_or("0").parse::<i32>().context("Failed to parse 'z' coordinate to i32")?;

            let location = Location { x, y: Some(y), z };
            markers_data.push(Marker { name, icon, location });
        }
    }

    Ok(markers_data)
}

pub async fn get_quest_count() -> anyhow::Result<i32> {
    let url = "https://api.wynncraft.com/v3/map/quests";

    let response = reqwest::get(url).await.context("Failed to make the API request")?;

    let data: Value = response.json().await.context("Failed to parse the JSON response")?;

    let quests = data["quests"].as_i64().ok_or_else(|| {
        anyhow::Error::msg("Expected 'quests' to be an integer")
    })? as i32;

    Ok(quests)
}