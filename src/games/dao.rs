use crate::games::Game;
use crate::utils::csv;

pub struct GamesDao {
    cache: Vec<Game>,
}

impl GamesDao {
    pub fn new() -> GamesDao {
        GamesDao {
            cache: csv::<Game>("resources/games.csv"),
        }
    }

    pub fn all(&self) -> Vec<Game> {
        self.cache.clone()
    }

    pub fn find_by_id(&self, id: i32) -> Option<Game> {
        self.cache.clone().into_iter().find(|it| it.id == id)
    }
}
