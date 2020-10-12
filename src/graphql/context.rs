use crate::games::dao::GamesDao;
use crate::missions::dao::MissionsDao;
use crate::service_records::dao::ServiceRecordsDao;

pub struct Context {
    pub games_dao: GamesDao,
    pub missions_dao: MissionsDao,
    pub service_records_doa: ServiceRecordsDao,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            games_dao: GamesDao::default(),
            missions_dao: MissionsDao::default(),
            service_records_doa: ServiceRecordsDao::default(),
        }
    }
}

impl juniper::Context for Context {}
