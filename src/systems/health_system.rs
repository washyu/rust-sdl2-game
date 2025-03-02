use crate::components::{Entity, Health};

pub struct HealthSystem;

impl HealthSystem {
    pub fn update(entities: &[Entity], _healths: &mut [Health]) {
        for (_i, _entity) in entities.iter().enumerate() {
            // Your code here
        }
    }
}