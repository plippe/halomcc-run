use strum::IntoEnumIterator;

use crate::games::game::Game;

pub struct GamesDao;

impl GamesDao {
    pub fn all(&self) -> Vec<Game> {
        Game::iter().collect()
    }

    pub fn find_by_id(&self, id: i32) -> Option<Game> {
        self.all().into_iter().find(|game| game.id() == id)
    }

    pub fn default() -> Self {
        GamesDao
    }
}
