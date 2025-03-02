pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(current: u32, max: u32) -> Self {
        Health { current, max }
    }
}