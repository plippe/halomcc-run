use crate::games::dao::{GamesDao, InMemoryGamesDao};
use crate::missions::dao::{InMemoryMissionsDao, MissionsDao};
use crate::service_records::dao::ServiceRecordsDao;

pub struct Context {
    games_dao: Box<dyn GamesDao + Send + Sync>,
    missions_dao: Box<dyn MissionsDao + Send + Sync>,
    service_records_doa: ServiceRecordsDao,
}

impl Context {
    pub fn games_dao(&self) -> &(dyn GamesDao + Send + Sync) {
        &*self.games_dao
    }

    pub fn missions_dao(&self) -> &(dyn MissionsDao + Send + Sync) {
        &*self.missions_dao
    }

    pub fn service_records_doa(&self) -> &ServiceRecordsDao {
        &self.service_records_doa
    }

    pub fn default() -> Self {
        Self {
            games_dao: Box::new(InMemoryGamesDao::default()),
            missions_dao: Box::new(InMemoryMissionsDao::default()),
            service_records_doa: ServiceRecordsDao::default(),
        }
    }
}

impl juniper::Context for Context {}
