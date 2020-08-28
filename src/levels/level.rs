use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Level {
    pub game_id: i32,
    pub id: i32,
    pub name: String,
    pub par_time: Option<i32>,
}
