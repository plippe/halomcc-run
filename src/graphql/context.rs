use crate::games::GamesDao;
use crate::levels::LevelsDao;
use crate::service_records::ServiceRecordsDao;

pub struct Context {
    pub games_dao: GamesDao,
    pub levels_dao: LevelsDao,
    pub service_records_doa: ServiceRecordsDao,
}

impl Context {
    pub fn new() -> Context {
        Context {
            games_dao: GamesDao::new(),
            levels_dao: LevelsDao::new(),
            service_records_doa: ServiceRecordsDao::new(),
        }
    }
}

impl juniper::Context for Context {}
