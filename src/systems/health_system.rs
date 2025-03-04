use crate::components::{Entity, Health};

pub struct HealthSystem;

impl HealthSystem {
    pub fn update(entities: &[Entity], healths: &mut [Health], delta_time: f32) {
        for (i, _entity) in entities.iter().enumerate() {
            if let Some(health) = healths.get_mut(i) {
                // Process damage queue
                if !health.damage_queue.is_empty() {
                    // Only apply damage if not invulnerable
                    if health.invulnerability_timer <= 0.0 {
                        for damage in health.damage_queue.drain(..) {
                            health.current = health.current.saturating_sub(damage);
                            
                            // Apply invulnerability frames
                            health.invulnerability_timer = health.invulnerability_duration;
                            
                            // Log damage for debugging
                            println!("Entity {} took {} damage, health now {}/{}", 
                                     i, damage, health.current, health.max);
                        }
                    }
                }
                
                // Process healing queue
                for healing in health.healing_queue.drain(..) {
                    health.current = (health.current + healing).min(health.max);
                    println!("Entity {} healed {} points, health now {}/{}", 
                             i, healing, health.current, health.max);
                }
                
                // Update invulnerability timer
                if health.invulnerability_timer > 0.0 {
                    health.invulnerability_timer -= delta_time;
                }
                
                // Natural regeneration (if enabled)
                if health.regeneration_rate > 0.0 && health.current < health.max {
                    health.regeneration_accumulator += health.regeneration_rate * delta_time;
                    
                    // Apply regeneration when accumulator reaches 1.0 or higher
                    if health.regeneration_accumulator >= 1.0 {
                        let regen_amount = health.regeneration_accumulator as u32;
                        health.current = (health.current + regen_amount).min(health.max);
                        health.regeneration_accumulator -= regen_amount as f32;
                    }
                }
                
                // Check for death
                if health.current == 0 && !health.is_dead {
                    health.is_dead = true;
                    println!("Entity {} has died!", i);
                    // You could emit an event or add to a "dead entities" list here
                }
            }
        }
    }
    
    // Convenience method to deal damage to an entity
    pub fn deal_damage(healths: &mut [Health], target_idx: usize, amount: u32) {
        if let Some(health) = healths.get_mut(target_idx) {
            health.damage_queue.push(amount);
        }
    }
    
    // Convenience method to heal an entity
    pub fn heal(healths: &mut [Health], target_idx: usize, amount: u32) {
        if let Some(health) = healths.get_mut(target_idx) {
            health.healing_queue.push(amount);
        }
    }
}