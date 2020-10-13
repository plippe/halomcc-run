use crate::games::dao::GamesDao;
use crate::missions::dao::MissionsDao;
use crate::service_records::dao::ServiceRecordsDao;

pub struct Context {
    pub games_dao: GamesDao,
    pub missions_dao: MissionsDao,
    pub service_records_doa: ServiceRecordsDao,
}

impl Context {
    pub fn new() -> Context {
        Context {
            games_dao: GamesDao::new(),
            missions_dao: MissionsDao::new(),
            service_records_doa: ServiceRecordsDao::new(),
        }
    }
}

impl juniper::Context for Context {}
