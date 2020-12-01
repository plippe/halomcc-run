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
    pub fn iter() -> impl Iterator<Item = Mission> {
        fn missions<Enum: IntoEnumIterator + Into<Mission>>() -> Vec<Mission> {
            Enum::iter().map(Into::into).collect()
        }

        [
            missions::<halo::HaloMission>(),
            missions::<halo_2::Halo2Mission>(),
            missions::<halo_3::Halo3Mission>(),
            missions::<halo_3_odst::Halo3OdstMission>(),
            missions::<halo_reach::HaloReachMission>(),
            missions::<halo_4::Halo4Mission>(),
        ]
        .concat()
        .into_iter()
    }

    pub fn game_id(self) -> i32 {
        MissionProperties::from(self).game_id
    }

    pub fn id(self) -> i32 {
        MissionProperties::from(self).id
    }

    pub fn name(self) -> String {
        MissionProperties::from(self).name
    }

    pub fn par_time(self) -> Option<Time> {
        MissionProperties::from(self).par_time
    }

    pub fn par_score(self) -> Option<i32> {
        MissionProperties::from(self).par_score
    }
}

struct MissionProperties {
    game_id: i32,
    id: i32,
    name: String,
    par_time: Option<Time>,
    par_score: Option<i32>,
}

impl From<Mission> for MissionProperties {
    fn from(mission: Mission) -> Self {
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
