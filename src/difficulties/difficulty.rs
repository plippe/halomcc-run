#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Heroic,
    Legendary,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => "easy".to_string(),
            Difficulty::Normal => "normal".to_string(),
            Difficulty::Heroic => "heroic".to_string(),
            Difficulty::Legendary => "legendary".to_string(),
        }
    }
}
