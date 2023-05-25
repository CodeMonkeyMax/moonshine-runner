pub struct RoundStateManager {
    current_state: RoundState,
}

pub enum RoundState {
    Drive,
    Barter,
    Buy,
    Brew,
}

impl RoundStateManager {
    pub fn new() {

    }
}