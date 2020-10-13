use time::Time;

mod halo;
mod halo_2;
mod halo_3;
mod halo_3_odst;
mod halo_4;
mod halo_reach;

use crate::chainable::Chainable;
use crate::games::game::{Game, GameProperties};

#[derive(Clone)]
pub enum Mission {
    Halo(halo::HaloMission),
    Halo2(halo_2::Halo2Mission),
    Halo3(halo_3::Halo3Mission),
    Halo3Odst(halo_3_odst::Halo3OdstMission),
    HaloReach(halo_reach::HaloReachMission),
    Halo4(halo_4::Halo4Mission),
}

impl Mission {
    pub fn all() -> Vec<Self> {
        fn into<A: Into<Mission>>(missions: Vec<A>) -> Vec<Mission> {
            missions.into_iter().map(Into::into).collect()
        }

        [
            halo::HaloMission::all().pipe(into),
            halo_2::Halo2Mission::all().pipe(into),
            halo_3::Halo3Mission::all().pipe(into),
            halo_3_odst::Halo3OdstMission::all().pipe(into),
            halo_reach::HaloReachMission::all().pipe(into),
            halo_4::Halo4Mission::all().pipe(into),
        ]
        .concat()
    }
}

pub struct MissionProperties {
    game_id: i32,
    id: i32,
    name: String,
    par_time: Option<Time>,
}

impl MissionProperties {
    fn new(game: Game, id: i32, name: String, par_time: Option<Time>) -> MissionProperties {
        MissionProperties {
            game_id: GameProperties::from(&game).id(),
            id,
            name,
            par_time,
        }
    }

    pub fn game_id(&self) -> i32 {
        self.game_id
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn par_time(&self) -> Option<Time> {
        self.par_time
    }
}

impl From<&Mission> for MissionProperties {
    fn from(mission: &Mission) -> Self {
        match mission {
            Mission::Halo(mission) => MissionProperties::from(mission),
            Mission::Halo2(mission) => MissionProperties::from(mission),
            Mission::Halo3(mission) => MissionProperties::from(mission),
            Mission::Halo3Odst(mission) => MissionProperties::from(mission),
            Mission::HaloReach(mission) => MissionProperties::from(mission),
            Mission::Halo4(mission) => MissionProperties::from(mission),
        }
    }
}
