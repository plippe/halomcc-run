use crate::games::game::{Game, GameId};

pub trait GamesDao {
    fn all(&self) -> Vec<Game>;
    fn find_by_id(&self, id: GameId) -> Option<Game>;
}

pub struct InMemoryGamesDao {
    games: Vec<Game>,
}

impl InMemoryGamesDao {
    pub fn default() -> Self {
        Self {
            games: vec![
                Game::new(GameId::new(1), "Halo: Combat Evolved Anniversary"),
                Game::new(GameId::new(2), "Halo 2: Anniversary"),
                Game::new(GameId::new(3), "Halo 3"),
                Game::new(GameId::new(4), "Halo 3: ODST"),
                Game::new(GameId::new(5), "Halo: Reach"),
                Game::new(GameId::new(6), "Halo 4"),
            ],
        }
    }
}

impl GamesDao for InMemoryGamesDao {
    fn all(&self) -> Vec<Game> {
        self.games.clone()
    }

    fn find_by_id(&self, id: GameId) -> Option<Game> {
        self.games.iter().find(|game| game.id() == id).cloned()
    }
}
