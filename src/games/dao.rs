use crate::games::game::{Game, GameProperties};

pub struct GamesDao;

impl GamesDao {
    pub fn new() -> GamesDao {
        GamesDao
    }

    pub fn all(&self) -> Vec<Game> {
        Game::all()
    }

    pub fn find_by_id(&self, id: i32) -> Option<Game> {
        Game::all()
            .into_iter()
            .find(|game| GameProperties::from(game).id() == id)
    }
}
