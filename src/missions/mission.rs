use strum::IntoEnumIterator;
use time::Time;

mod halo;
mod halo_2;
mod halo_3;
mod halo_3_odst;
mod halo_4;
mod halo_reach;

#[derive(Clone, Copy)]
pub enum Mission {
    Halo(halo::HaloMission),
    Halo2(halo_2::Halo2Mission),
    Halo3(halo_3::Halo3Mission),
    Halo3Odst(halo_3_odst::Halo3OdstMission),
    HaloReach(halo_reach::HaloReachMission),
    Halo4(halo_4::Halo4Mission),
}

impl Mission {
    #[rustfmt::skip]
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            halo::HaloMission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
            halo_2::Halo2Mission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
            halo_3::Halo3Mission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
            halo_3_odst::Halo3OdstMission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
            halo_reach::HaloReachMission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
            halo_4::Halo4Mission::iter().map(|m| m.to_mission()).collect::<Vec<Self>>(),
        ]
        .concat()
        .into_iter()
    }

    pub fn game_id(&self) -> i32 {
        self.to_properties().game_id
    }

    pub fn id(&self) -> i32 {
        self.to_properties().id
    }

    pub fn name(&self) -> String {
        self.to_properties().name
    }

    pub fn par_time(&self) -> Option<Time> {
        self.to_properties().par_time
    }

    pub fn par_score(&self) -> Option<i32> {
        self.to_properties().par_score
    }

    #[rustfmt::skip]
    fn to_properties(&self) -> MissionProperties {
        match self {
            Self::Halo(mission) => mission.to_properties(),
            Self::Halo2(mission) => mission.to_properties(),
            Self::Halo3(mission) => mission.to_properties(),
            Self::Halo3Odst(mission) => mission.to_properties(),
            Self::HaloReach(mission) => mission.to_properties(),
            Self::Halo4(mission) => mission.to_properties(),
        }
    }
}

pub struct MissionProperties {
    game_id: i32,
    id: i32,
    name: String,
    par_time: Option<Time>,
    par_score: Option<i32>,
}
