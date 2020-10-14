use time::Time;

pub struct ServiceRecord {
    player: String,
    game_id: i32,
    mission_id: i32,
    time: Option<Time>,
}

impl ServiceRecord {
    pub fn new(player: String, game_id: i32, mission_id: i32, time: Option<Time>) -> ServiceRecord {
        ServiceRecord {
            player,
            game_id,
            mission_id,
            time,
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

    pub fn time(&self) -> Option<Time> {
        self.time
    }
}
