use time::Time;

use crate::games::game::GameId;

#[derive(Clone)]
pub struct Mission {
    game_id: GameId,
    id: i32,
    name: String,
    par_time: Option<Time>,
    par_score: Option<i32>,
}

impl Mission {
    pub fn new(
        game_id: GameId,
        id: i32,
        name: &str,
        par_time: Option<Time>,
        par_score: Option<i32>,
    ) -> Self {
        Self {
            game_id,
            id,
            name: name.to_string(),
            par_time,
            par_score,
        }
    }

    pub fn game_id(&self) -> GameId {
        self.game_id
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn par_time(&self) -> Option<Time> {
        self.par_time
    }

    pub fn par_score(&self) -> Option<i32> {
        self.par_score
    }
}
