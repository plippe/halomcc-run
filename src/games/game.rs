use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(i32);

impl GameId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &i32 {
        &self.0
    }
}

impl Ord for GameId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(other.value())
    }
}

impl PartialOrd for GameId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    id: GameId,
    name: String,
}

impl Game {
    pub fn new(id: GameId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> GameId {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
