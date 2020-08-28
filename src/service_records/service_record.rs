use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServiceRecord {
    pub player: String,
    pub game_id: i32,
    pub level_id: i32,
    pub best_time: i32,
}
