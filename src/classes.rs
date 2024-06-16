use anyhow::Context;
use serde_json::Value;

#[derive(Debug)]
pub struct SimpleClass {
    pub id: String,
    pub name: String,
    pub overall_difficulty: i32,
}

#[derive(Debug)]
pub struct ClassData {
    pub id: String,
    pub name: String,
    pub lore: String,
    pub overall_difficulty: i32,
    pub archetypes: Vec<Archetype>,
}

#[derive(Debug)]
pub struct Archetype {
    pub id: String,
    pub name: String,
    pub difficulty: i32,
    pub damage: i32,
    pub defence: i32,
    pub range: i32,
    pub speed: i32,
}

pub async fn get_class(id: &str) -> anyhow::Result<ClassData> {
    let url = format!("https://api.wynncraft.com/v3/classes/{}", id);

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let class_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let id = class_object["id"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'id' to be a String"))?
        .parse::<String>()
        .unwrap();

    let name = class_object["name"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'name' to be a String"))?
        .parse::<String>()
        .unwrap();

    let lore = class_object["lore"]
        .as_str()
        .ok_or_else(|| anyhow::Error::msg("Expected 'lore' to be a String"))?
        .parse::<String>()
        .unwrap();

    let overall_difficulty = class_object["overallDifficulty"]
        .as_i64()
        .ok_or_else(|| anyhow::Error::msg("Expected 'overallDifficulty' to be an integer"))?
        as i32;

    let mut archetypes = Vec::new();
    let archetypes_object = class_object
        .get("archetypes")
        .ok_or_else(|| anyhow::Error::msg("Expected 'archetypes' to be a JSON object"))?
        .as_object()
        .unwrap();

    for (archetype_id, archetype_details) in archetypes_object {
        let archetype_name = archetype_details
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let difficulty = archetype_details
            .get("difficulty")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        let damage = archetype_details
            .get("damage")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        let defence = archetype_details
            .get("defence")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        let range = archetype_details
            .get("range")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;
        let speed = archetype_details
            .get("speed")
            .and_then(Value::as_i64)
            .unwrap_or_default() as i32;

        archetypes.push(Archetype {
            id: archetype_id.to_string(),
            name: archetype_name,
            difficulty,
            damage,
            defence,
            range,
            speed,
        });
    }

    Ok(ClassData {
        id,
        name,
        lore,
        overall_difficulty,
        archetypes,
    })
}

pub async fn get_classes() -> anyhow::Result<Vec<SimpleClass>> {
    let url = "https://api.wynncraft.com/v3/classes";

    let response = reqwest::get(url)
        .await
        .context("Failed to make the API request")?;
    let data: Value = response
        .json()
        .await
        .context("Failed to parse the JSON response")?;

    let classes_object = data
        .as_object()
        .ok_or_else(|| anyhow::Error::msg("Expected the JSON to be an object"))?;

    let mut class_data = Vec::new();
    for (id, details) in classes_object {
        if let Some(class_details) = details.as_object() {
            let name = class_details
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let overall_difficulty = class_details
                .get("overallDifficulty")
                .and_then(Value::as_i64)
                .unwrap_or_default() as i32;
            class_data.push(SimpleClass {
                id: id.clone(),
                name,
                overall_difficulty,
            });
        }
    }

    Ok(class_data)
}
