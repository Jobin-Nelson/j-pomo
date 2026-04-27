use std::fmt::Display;

use crate::constants::*;

pub type PomoProgress = f64;
pub type RemainingSecs = u32;

#[derive(Default)]
pub enum PomoKind {
    #[default]
    Focus,
    Break,
    LongBreak,
}

impl PomoKind {
    pub fn get_mins(&self) -> u32 {
        match self {
            PomoKind::Focus => FOCUS_MINS,
            PomoKind::Break => BREAK_MINS,
            PomoKind::LongBreak => LONG_BREAK_MINS,
        }
    }
}

impl Display for PomoKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PomoKind::Focus => write!(f, "FOCUS"),
            PomoKind::Break => write!(f, "BREAK"),
            PomoKind::LongBreak => write!(f, "LONG BREAK"),
        }
    }
}

#[derive(Debug, Default)]
pub enum PomoStatus {
    #[default]
    Running,
    Paused,
}

#[derive(Default)]
pub struct Pomo {
    pub kind: PomoKind,
    pub count: usize,
    pub progress: PomoProgress,
    pub status: PomoStatus,
    pub rem: RemainingSecs,
}

impl Pomo {
    pub fn new() -> Self {
        Default::default()
    }
}
