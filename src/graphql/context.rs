use crate::games::GamesDao;
use crate::levels::LevelsDao;

pub struct Context {
    pub games_dao: GamesDao,
    pub levels_dao: LevelsDao,
}

impl Context {
    pub fn new() -> Context {
        Context {
            games_dao: GamesDao::new(),
            levels_dao: LevelsDao::new(),
        }
    }
}

impl juniper::Context for Context {}
