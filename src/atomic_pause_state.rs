use std::sync::atomic::{AtomicI8, Ordering};

use crate::pause_state::PauseState;

#[derive(Debug)]
pub struct AtomicPauseState(AtomicI8);

impl AtomicPauseState {
    pub const fn new() -> Self {
        Self(AtomicI8::new(PauseState::Unknown as i8))
    }

    pub fn load(&self) -> PauseState {
        match self.0.load(Ordering::Relaxed) {
            0 => PauseState::Resumed,
            1 => PauseState::Paused,
            _ => PauseState::Unknown,
        }
    }

    pub fn store(&self, state: PauseState) {
        self.0.store(state as i8, Ordering::Relaxed);
    }
}
