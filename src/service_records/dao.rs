use crate::service_records::service_record::ServiceRecord;

pub struct ServiceRecordsDao;

impl ServiceRecordsDao {
    pub fn new() -> ServiceRecordsDao {
        ServiceRecordsDao
    }

    pub fn find_by_player_game_id_and_mission_id(
        &self,
        player: String,
        game_id: i32,
        mission_id: i32,
    ) -> Option<ServiceRecord> {
        Some(ServiceRecord::new(player, game_id, mission_id, None))
    }
}
