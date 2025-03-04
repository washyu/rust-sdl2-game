#[derive(Clone)]
pub struct Health {
    pub current: u32,
    pub max: u32,
    pub damage_queue: Vec<u32>,
    pub healing_queue: Vec<u32>,
    pub invulnerability_timer: f32,
    pub invulnerability_duration: f32,
    pub regeneration_rate: f32,       // Health points regenerated per second
    pub regeneration_accumulator: f32, // Used for fractional regeneration
    pub is_dead: bool,
}

impl Health {
    pub fn new(current: u32, max: u32) -> Self {
        Health {
            current,
            max,
            damage_queue: Vec::new(),
            healing_queue: Vec::new(),
            invulnerability_timer: 0.0,
            invulnerability_duration: 0.5,  // 0.5 seconds of invulnerability after taking damage
            regeneration_rate: 0.0,         // No regeneration by default
            regeneration_accumulator: 0.0,
            is_dead: false,
        }
    }
    
    // Create with custom invulnerability duration
    pub fn with_invulnerability(current: u32, max: u32, invulnerability_duration: f32) -> Self {
        let mut health = Health::new(current, max);
        health.invulnerability_duration = invulnerability_duration;
        health
    }
    
    // Create with regeneration
    pub fn with_regeneration(current: u32, max: u32, regen_rate: f32) -> Self {
        let mut health = Health::new(current, max);
        health.regeneration_rate = regen_rate;
        health
    }
    
    // Is this entity currently invulnerable?
    pub fn is_invulnerable(&self) -> bool {
        self.invulnerability_timer > 0.0
    }
    
    // Visual indicator for damage - flashes when recently hit
    pub fn is_flashing(&self) -> bool {
        self.is_invulnerable() && (self.invulnerability_timer * 10.0) as i32 % 2 == 0
    }
}