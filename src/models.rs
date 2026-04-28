use std::fmt::Display;
use std::path::PathBuf;

use crate::{Result, config::create_state_file, constants::*};

use ratatui::{
    text::Line,
    widgets::{Block, Borders},
};
use ratatui_textarea::TextArea;

pub type PomoProgress = f64;
pub type RemainingSecs = u32;

#[derive(Debug, Default)]
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
    Done,
}

#[derive(Debug, Default)]
pub enum AppMode {
    #[default]
    Progress,
    SessionName,
}

#[derive(Debug, Default)]
pub struct Pomo {
    pub kind: PomoKind,
    pub count: usize,
    pub progress: PomoProgress,
    pub status: PomoStatus,
    pub rem: RemainingSecs,
    pub session: Option<String>,
}

impl Pomo {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct StateFileGuard {
    pub path: PathBuf,
}

impl Drop for StateFileGuard {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub should_exit: bool,
    pub pomo: Pomo,
    pub session_name: TextArea<'static>,
    pub mode: AppMode,
    pub state_file: StateFileGuard,
}

impl App {
    pub fn new() -> Result<Self> {
        let state_file = StateFileGuard {
            path: create_state_file()?,
        };

        let mut app = App {
            state_file,
            should_exit: Default::default(),
            pomo: Default::default(),
            session_name: Default::default(),
            mode: Default::default(),
        };

        app.session_name.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" Session Name ").centered()),
        );
        Ok(app)
    }
}
