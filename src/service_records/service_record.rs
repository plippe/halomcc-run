use time::Time;

use std::cmp::Ordering;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::difficulties::difficulty::Difficulty;

#[derive(PartialEq, Eq)]
pub struct ServiceRecord {
    player: String,
    game_id: i32,
    mission_id: i32,
    runs: Vec<ServiceRecordRun>,
}

impl ServiceRecord {
    pub fn new(
        player: String,
        game_id: i32,
        mission_id: i32,
        runs: Vec<ServiceRecordRun>,
    ) -> ServiceRecord {
        ServiceRecord {
            player,
            game_id,
            mission_id,
            runs,
        }
    }

    pub fn player(&self) -> String {
        self.player.clone()
    }

    pub fn game_id(&self) -> i32 {
        self.game_id
    }

    pub fn mission_id(&self) -> i32 {
        self.mission_id
    }

    pub fn runs(&self) -> Vec<ServiceRecordRun> {
        self.runs.clone()
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
