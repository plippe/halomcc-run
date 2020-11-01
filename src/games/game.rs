use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Game {
    Halo,
    Halo2,
    Halo3,
    Halo3Odst,
    HaloReach,
    Halo4,
}

pub struct GameProperties {
    id: i32,
    name: String,
}

impl GameProperties {
    fn new(id: i32, name: String) -> GameProperties {
        GameProperties { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<Game> for GameProperties {
    fn from(game: Game) -> GameProperties {
        match game {
            Game::Halo => GameProperties::new(1, "Halo: Combat Evolved Anniversary".to_string()),
            Game::Halo2 => GameProperties::new(2, "Halo 2: Anniversary".to_string()),
            Game::Halo3 => GameProperties::new(3, "Halo 3".to_string()),
            Game::Halo3Odst => GameProperties::new(4, "Halo 3: ODST".to_string()),
            Game::HaloReach => GameProperties::new(5, "Halo: Reach".to_string()),
            Game::Halo4 => GameProperties::new(6, "Halo 4".to_string()),
        }
    }
}
