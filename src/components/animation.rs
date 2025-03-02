#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Walk,
    Attack,
    Hurt,
    Death,
}

pub struct Animation {
    pub current_frame: usize,
    pub frame_count: usize,
    pub frame_duration: u32, // Duration per frame in milliseconds
    pub state: AnimationState,
    pub elapsed_time: u32,   // Track elapsed time for animation updates
    pub is_attack_in_progress: bool,  // Add this flag
}

impl Animation {
    pub fn new(frame_count: usize, state: AnimationState) -> Self {
        Animation { 
            current_frame: 0, 
            frame_count, 
            frame_duration: 100,
            state,
            elapsed_time: 0,
            is_attack_in_progress: false,  // Initialize as false
        }
    }
    
    pub fn update(&mut self, delta_time: u32) {
        self.elapsed_time += delta_time;
        if self.elapsed_time >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frame_count;
            self.elapsed_time = 0;
            
            // Check if attack animation has completed
            if self.is_attack_in_progress && self.state == AnimationState::Attack && self.current_frame == 0 {
                self.is_attack_in_progress = false;
            }
        }
    }
}