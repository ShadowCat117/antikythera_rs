mod player;
mod guild;
mod map;
mod leaderboard;

use serde_json::Value;
pub use player::*;
pub use guild::*;
pub use map::*;
pub use leaderboard::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Location {
    x: i32,
    y: Option<i32>,
    z: i32
}

fn json_to_location(value: &Value) -> Location {
    let loc = value.as_array().unwrap();

    if loc.len() == 3 {
        Location {
            x: loc[0].as_i64().unwrap() as i32,
            y: Some(loc[1].as_i64().unwrap() as i32),
            z: loc[2].as_i64().unwrap() as i32,
        }
    } else {
        Location {
            x: loc[0].as_i64().unwrap() as i32,
            y: None,
            z: loc[1].as_i64().unwrap() as i32,
        }
    }
}
