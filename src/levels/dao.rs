use crate::levels::Level;
use crate::utils::csv;

pub struct LevelsDao {
    cache: Vec<Level>,
}

impl LevelsDao {
    pub fn new() -> LevelsDao {
        LevelsDao {
            cache: csv::<Level>("resources/levels.csv"),
        }
    }

    pub fn find_by_game_id_and_id(&self, game_id: i32, id: i32) -> Option<Level> {
        self.cache
            .clone()
            .into_iter()
            .find(|it| it.game_id == game_id && it.id == id)
    }

    pub fn all_by_game_id(&self, game_id: i32) -> Vec<Level> {
        self.cache
            .clone()
            .into_iter()
            .filter(|it| it.game_id == game_id)
            .collect()
    }
}
