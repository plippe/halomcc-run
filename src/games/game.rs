use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Game {
    pub id: i32,
    pub name: String,
}
