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

impl Game {
    pub fn id(self) -> i32 {
        GameProperties::from(self).id
    }

    pub fn name(self) -> String {
        GameProperties::from(self).name
    }
}

struct GameProperties {
    id: i32,
    name: String,
}

impl From<Game> for GameProperties {
    #[rustfmt::skip]
    fn from(game: Game) -> GameProperties {
        match game {
            Game::Halo => GameProperties { id: 1, name: "Halo: Combat Evolved Anniversary".to_string() },
            Game::Halo2 => GameProperties { id: 2, name: "Halo 2: Anniversary".to_string() },
            Game::Halo3 => GameProperties { id: 3, name: "Halo 3".to_string() },
            Game::Halo3Odst => GameProperties { id: 4, name: "Halo 3: ODST".to_string() },
            Game::HaloReach => GameProperties { id: 5, name: "Halo: Reach".to_string() },
            Game::Halo4 => GameProperties { id: 6, name: "Halo 4".to_string() },
        }
    }
}
