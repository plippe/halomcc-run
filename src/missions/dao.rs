use itertools::Itertools;
use time::time;

use crate::games::game::GameId;
use crate::missions::mission::{Mission, MissionId};

pub trait MissionsDao {
    fn all_by_game_id(&self, game_id: GameId) -> Vec<Mission>;
    fn find_by_game_id_and_id(&self, game_id: GameId, id: MissionId) -> Option<Mission>;
}

pub struct InMemoryMissionsDao {
    missions: Vec<Mission>,
}

impl InMemoryMissionsDao {
    pub fn default() -> Self {
        #[rustfmt::skip]
        let halo = vec![
            Mission::new(GameId::new(1), MissionId::new(1), "Pillar of Autumn", Some(time!(00:15:00)), Some(17_000)),
            Mission::new(GameId::new(1), MissionId::new(2), "Halo", Some(time!(00:20:00)), Some(12_000)),
            Mission::new(GameId::new(1), MissionId::new(3), "Truth and Reconciliation", Some(time!(00:20:00)), Some(19_000)),
            Mission::new(GameId::new(1), MissionId::new(4), "Silent Cartographer", Some(time!(00:15:00)), Some(18_000)),
            Mission::new(GameId::new(1), MissionId::new(5), "Assault on the Control Room", Some(time!(00:15:00)), Some(18_000)),
            Mission::new(GameId::new(1), MissionId::new(6), "343 Guilty Spark", Some(time!(00:15:00)), Some(17_000)),
            Mission::new(GameId::new(1), MissionId::new(7), "The Library", Some(time!(00:25:00)), Some(25_000)),
            Mission::new(GameId::new(1), MissionId::new(8), "Two Betrayals", Some(time!(00:20:00)), Some(16_000)),
            Mission::new(GameId::new(1), MissionId::new(9), "Keyes", Some(time!(00:15:00)), Some(20_000)),
            Mission::new(GameId::new(1), MissionId::new(10), "The Maw", Some(time!(00:15:00)), Some(18_000)),
        ];

        #[rustfmt::skip]
        let halo_2 = vec![
            Mission::new(GameId::new(2), MissionId::new(1), "The Heretic", None, None),
            Mission::new(GameId::new(2), MissionId::new(2), "The Armory", None, None),
            Mission::new(GameId::new(2), MissionId::new(3), "Cairo Station", Some(time!(00:15:00)), Some(14_000)),
            Mission::new(GameId::new(2), MissionId::new(4), "Outskirts", Some(time!(00:15:00)), Some(8_000)),
            Mission::new(GameId::new(2), MissionId::new(5), "Metropolis", Some(time!(00:15:00)), Some(9_000)),
            Mission::new(GameId::new(2), MissionId::new(6), "The Arbiter", Some(time!(00:15:00)), Some(7_000)),
            Mission::new(GameId::new(2), MissionId::new(7), "The Oracle", Some(time!(00:25:00)), Some(16_000)),
            Mission::new(GameId::new(2), MissionId::new(8), "Delta Halo", Some(time!(00:15:00)), Some(10_000)),
            Mission::new(GameId::new(2), MissionId::new(9), "Regret", Some(time!(00:15:00)), Some(8_000)),
            Mission::new(GameId::new(2), MissionId::new(10), "Sacred Icon", Some(time!(00:15:00)), Some(7_000)),
            Mission::new(GameId::new(2), MissionId::new(11), "Quarantine Zone", Some(time!(00:15:00)), Some(7_000)),
            Mission::new(GameId::new(2), MissionId::new(12), "Gravemind", Some(time!(00:20:00)), Some(11_000)),
            Mission::new(GameId::new(2), MissionId::new(13), "Uprising", Some(time!(00:15:00)), Some(9_000)),
            Mission::new(GameId::new(2), MissionId::new(14), "High Charity", Some(time!(00:15:00)), Some(9_000)),
            Mission::new(GameId::new(2), MissionId::new(15), "The Great Journey", Some(time!(00:15:00)), Some(8_000)),
        ];

        #[rustfmt::skip]
        let halo_3 = vec![
            Mission::new(GameId::new(3), MissionId::new(1), "Arrival", None, None),
            Mission::new(GameId::new(3), MissionId::new(2), "Sierra 117", Some(time!(00:15:00)), Some(13_000)),
            Mission::new(GameId::new(3), MissionId::new(3), "Crow’s Nest", Some(time!(00:20:00)), Some(19_000)),
            Mission::new(GameId::new(3), MissionId::new(4), "Tvaso Highway", Some(time!(00:20:00)), Some(21_000)),
            Mission::new(GameId::new(3), MissionId::new(5), "The Storm", Some(time!(00:15:00)), Some(15_000)),
            Mission::new(GameId::new(3), MissionId::new(6), "Floodgate", Some(time!(00:15:00)), Some(25_000)),
            Mission::new(GameId::new(3), MissionId::new(7), "The Ark", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(3), MissionId::new(8), "The Covenant", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(3), MissionId::new(9), "Cortana", Some(time!(00:15:00)), Some(17_000)),
            Mission::new(GameId::new(3), MissionId::new(10), "Halo", Some(time!(00:20:00)), Some(24_000)),
            Mission::new(GameId::new(3), MissionId::new(11), "Epilogue", None, None),
        ];

        #[rustfmt::skip]
        let halo_3_odst = vec![
            Mission::new(GameId::new(4), MissionId::new(1), "Prepare To Drop", None, None),
            Mission::new(GameId::new(4), MissionId::new(2), "Mombasa Streets", None, None),
            Mission::new(GameId::new(4), MissionId::new(3), "Tayari Plaza", Some(time!(00:03:00)), Some(8_000)),
            Mission::new(GameId::new(4), MissionId::new(4), "Uplift Reserve", Some(time!(00:04:00)), Some(14_000)),
            Mission::new(GameId::new(4), MissionId::new(5), "Kizingo Boulevard", Some(time!(00:09:00)), Some(18_000)),
            Mission::new(GameId::new(4), MissionId::new(6), "ONI Alpha Site", Some(time!(00:13:00)), Some(16_000)),
            Mission::new(GameId::new(4), MissionId::new(7), "NMPD HQ", Some(time!(00:10:00)), Some(40_000)),
            Mission::new(GameId::new(4), MissionId::new(8), "Kikowani Station", Some(time!(00:10:00)), Some(42_000)),
            Mission::new(GameId::new(4), MissionId::new(9), "Data Hive", Some(time!(00:16:00)), Some(8_000)),
            Mission::new(GameId::new(4), MissionId::new(10), "Coastal Highway", Some(time!(00:25:00)), Some(90_000)),
            Mission::new(GameId::new(4), MissionId::new(11), "Epilogue", None, None),
        ];

        #[rustfmt::skip]
        let halo_reach = vec![
            Mission::new(GameId::new(5), MissionId::new(1), "Noble Actual", None, None),
            Mission::new(GameId::new(5), MissionId::new(2), "Winter Contingency", Some(time!(00:15:00)), Some(15_000)),
            Mission::new(GameId::new(5), MissionId::new(3), "ONI Sword Base", Some(time!(00:10:00)), Some(25_000)),
            Mission::new(GameId::new(5), MissionId::new(4), "Nightfall", Some(time!(00:10:00)), Some(7_500)),
            Mission::new(GameId::new(5), MissionId::new(5), "Tip of The Spear", Some(time!(00:15:00)), Some(30_000)),
            Mission::new(GameId::new(5), MissionId::new(6), "Long Night of Solace", Some(time!(00:25:00)), Some(45_000)),
            Mission::new(GameId::new(5), MissionId::new(7), "Exodus", Some(time!(00:20:00)), Some(30_000)),
            Mission::new(GameId::new(5), MissionId::new(8), "New Alexandria", Some(time!(00:20:00)), Some(22_500)),
            Mission::new(GameId::new(5), MissionId::new(9), "The Package", Some(time!(00:20:00)), Some(65_000)),
            Mission::new(GameId::new(5), MissionId::new(10), "The Pillar of Autumn", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(5), MissionId::new(11), "Epilogue", None, None),
            Mission::new(GameId::new(5), MissionId::new(12), "Lone Wolf", None, None),
        ];

        #[rustfmt::skip]
        let halo_4 = vec![
            Mission::new(GameId::new(6), MissionId::new(1), "Prologue", None, None),
            Mission::new(GameId::new(6), MissionId::new(2), "Dawn", Some(time!(00:15:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(3), "Requiem", Some(time!(00:15:00)), Some(22_000)),
            Mission::new(GameId::new(6), MissionId::new(4), "Forerunner", Some(time!(00:20:00)), Some(22_000)),
            Mission::new(GameId::new(6), MissionId::new(5), "Infinity", Some(time!(00:25:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(6), "Reclaimer", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(7), "Shutdown", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(8), "Composer", Some(time!(00:20:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(9), "Midnight", Some(time!(00:25:00)), Some(25_000)),
            Mission::new(GameId::new(6), MissionId::new(10), "Epilogue", None, None),
        ];

        let missions = vec![halo, halo_2, halo_3, halo_3_odst, halo_reach, halo_4]
            .into_iter()
            .concat();

        Self { missions }
    }
}

impl MissionsDao for InMemoryMissionsDao {
    fn all_by_game_id(&self, game_id: GameId) -> Vec<Mission> {
        self.missions
            .iter()
            .filter(|mission| mission.game_id() == game_id)
            .cloned()
            .collect()
    }

    fn find_by_game_id_and_id(&self, game_id: GameId, id: MissionId) -> Option<Mission> {
        self.missions
            .iter()
            .find(|mission| mission.game_id() == game_id && mission.id() == id)
            .cloned()
    }
}
