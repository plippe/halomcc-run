use std::cmp::Ordering;
use time::Time;

use crate::games::game::GameId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MissionId(i32);

impl MissionId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &i32 {
        &self.0
    }
}

impl Ord for MissionId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(other.value())
    }
}

impl PartialOrd for MissionId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct Mission {
    game_id: GameId,
    id: MissionId,
    name: String,
    par_time: Option<Time>,
    par_score: Option<i32>,
}

impl Mission {
    pub fn new(
        game_id: GameId,
        id: MissionId,
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

    pub fn id(&self) -> MissionId {
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
