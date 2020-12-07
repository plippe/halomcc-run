use crate::games::game::Game;

pub struct GamesDao;
impl GamesDao {
    pub fn all(&self) -> Vec<Game> {
        vec![
            Game::new(1, "Halo: Combat Evolved Anniversary"),
            Game::new(2, "Halo 2: Anniversary"),
            Game::new(3, "Halo 3"),
            Game::new(4, "Halo 3: ODST"),
            Game::new(5, "Halo: Reach"),
            Game::new(6, "Halo 4"),
        ]
    }

    pub fn find_by_id(&self, id: i32) -> Option<Game> {
        self.all().into_iter().find(|game| game.id() == id)
    }

    pub fn default() -> Self {
        Self
    }
}
