use crate::missions::mission::{Mission, MissionProperties};

pub struct MissionsDao;

impl MissionsDao {
    pub fn all_by_game_id(&self, game_id: i32) -> Vec<Mission> {
        Mission::iter()
            .filter(|mission| MissionProperties::from(mission).game_id() == game_id)
            .collect()
    }

    pub fn find_by_game_id_and_id(&self, game_id: i32, id: i32) -> Option<Mission> {
        Mission::iter().find(|mission| {
            let properties = MissionProperties::from(mission);
            properties.game_id() == game_id && properties.id() == id
        })
    }
}

impl Default for MissionsDao {
    fn default() -> Self {
        MissionsDao
    }
}
