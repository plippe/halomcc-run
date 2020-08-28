use crate::service_records::ServiceRecord;
use crate::utils::csv;

pub struct ServiceRecordsDao {
    cache: Vec<ServiceRecord>,
}

impl ServiceRecordsDao {
    pub fn new() -> ServiceRecordsDao {
        ServiceRecordsDao {
            cache: csv::<ServiceRecord>("resources/service_records.csv"),
        }
    }

    pub fn find_by_player_game_id_and_level_id(
        &self,
        player: String,
        game_id: i32,
        level_id: i32,
    ) -> Option<ServiceRecord> {
        self.cache
            .clone()
            .into_iter()
            .find(|it| it.player == player && it.game_id == game_id && it.level_id == level_id)
    }
}
