#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum PauseState {
    Unknown = -1,
    Resumed = 0,
    Paused = 1,
}

impl From<bool> for PauseState {
    fn from(paused: bool) -> Self {
        if paused {
            PauseState::Paused
        } else {
            PauseState::Resumed
        }
    }
}
