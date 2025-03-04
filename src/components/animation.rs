#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Walk,
    Attack,
    Hurt,
    Death,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub current_frame: usize,
    pub frame_duration: u32, // Duration per frame in milliseconds
    pub state: AnimationState,
    pub elapsed_time: u32,   // Track elapsed time for animation updates
    pub is_attack_in_progress: bool,
    
    // Frame counts for each animation type
    pub idle_frames: usize,
    pub walk_frames: usize,
    pub attack_frames: usize,
}

impl Animation {
    pub fn new(state: AnimationState, idle_frames: usize, walk_frames: usize, attack_frames: usize) -> Self {
        Animation { 
            current_frame: 0,
            frame_duration: 100,
            state,
            elapsed_time: 0,
            is_attack_in_progress: false,
            idle_frames,
            walk_frames,
            attack_frames,
        }
    }
    
    pub fn update(&mut self, frame_time: u32) {
        self.elapsed_time += frame_time;
        
        // Update frame if enough time has passed
        if self.elapsed_time >= self.frame_duration {
            self.elapsed_time = 0;
            self.current_frame += 1;
            
            // Get total frames for current animation state
            let total_frames = match self.state {
                AnimationState::Idle => self.idle_frames,
                AnimationState::Walk => self.walk_frames,
                AnimationState::Attack => self.attack_frames,
                _ => self.idle_frames, // Default to idle for other states
            };
            
            // Loop animation, but also reset attack flag when attack animation completes
            if self.current_frame >= total_frames {
                self.current_frame = 0;
                
                // Reset attack flag when attack animation completes one cycle
                if self.state == AnimationState::Attack && self.is_attack_in_progress {
                    println!("Attack animation completed!");
                    self.is_attack_in_progress = false;
                    self.state = AnimationState::Idle;  // Return to idle after attack
                }
            }
        }
    }

    pub fn has_completed_cycle(&self) -> bool {
        let total_frames = match self.state {
            AnimationState::Idle => self.idle_frames,
            AnimationState::Walk => self.walk_frames,
            AnimationState::Attack => self.attack_frames,
            _ => self.idle_frames,
        };
        
        self.current_frame >= total_frames - 1
    }
}