use itertools::Itertools;
use time::Time;

use std::cmp::Ordering;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::difficulties::difficulty::Difficulty;
use crate::games::game::GameId;

#[derive(PartialEq, Eq)]
pub struct ServiceRecord {
    player: String,
    game_id: GameId,
    mission_id: i32,
    runs: Vec<ServiceRecordRun>,
}

impl ServiceRecord {
    pub fn new(
        player: String,
        game_id: GameId,
        mission_id: i32,
        runs: Vec<ServiceRecordRun>,
    ) -> Self {
        Self {
            player,
            game_id,
            mission_id,
            runs,
        }
    }

    pub fn player(&self) -> String {
        self.player.clone()
    }

    pub fn game_id(&self) -> GameId {
        self.game_id
    }

    pub fn mission_id(&self) -> i32 {
        self.mission_id
    }

    pub fn runs(&self) -> Vec<ServiceRecordRun> {
        self.runs.clone()
    }

    pub fn from_player_and_runs(
        player: &str,
        runs: &[(GameId, i32, CampaignMode, Difficulty, Time, i32)],
    ) -> Vec<Self> {
        runs.iter()
            .map(|(g, m, c, d, t, s)| ((g, m), (c, d, t, s)))
            .into_group_map()
            .into_iter()
            .map(|((game_id, mission_id), runs)| {
                let runs = runs
                    .into_iter()
                    .map(|(c, d, t, s)| ServiceRecordRun::new(*c, *d, *t, *s))
                    .collect();

                Self::new(player.to_string(), *game_id, *mission_id, runs)
            })
            .sorted()
            .collect()
    }
}

impl Ord for ServiceRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.game_id, &self.mission_id).cmp(&(other.game_id, &other.mission_id))
    }
}

impl PartialOrd for ServiceRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ServiceRecordRun {
    campaign_mode: CampaignMode,
    difficulty: Difficulty,
    time: Time,
    score: i32,
}

impl ServiceRecordRun {
    pub fn new(
        campaign_mode: CampaignMode,
        difficulty: Difficulty,
        time: Time,
        score: i32,
    ) -> Self {
        Self {
            campaign_mode,
            difficulty,
            time,
            score,
        }
    }

    pub fn campaign_mode(&self) -> CampaignMode {
        self.campaign_mode
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn time(&self) -> Time {
        self.time
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}
