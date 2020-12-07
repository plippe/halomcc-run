#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    id: i32,
    name: String,
}

impl Game {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
