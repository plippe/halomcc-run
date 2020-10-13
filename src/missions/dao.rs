use crate::missions::mission::{Mission, MissionProperties};

pub struct MissionsDao;

impl MissionsDao {
    pub fn new() -> MissionsDao {
        MissionsDao
    }

    pub fn all_by_game_id(&self, game_id: i32) -> Vec<Mission> {
        Mission::all()
            .into_iter()
            .filter(|mission| MissionProperties::from(mission).game_id() == game_id)
            .collect()
    }
}
