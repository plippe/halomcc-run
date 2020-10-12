use crate::missions::mission::{Mission, MissionProperties};

pub struct MissionsDao;

impl MissionsDao {
    pub fn all_by_game_id(&self, game_id: i32) -> Vec<Mission> {
        Mission::iter()
            .filter(|mission| MissionProperties::from(mission).game_id() == game_id)
            .collect()
    }
}

impl Default for MissionsDao {
    fn default() -> Self {
        MissionsDao
    }
}
