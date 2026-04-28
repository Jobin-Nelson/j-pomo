use std::fmt::Display;

use ratatui::{
    text::Line,
    widgets::{Block, Borders},
};
use ratatui_textarea::TextArea;

use crate::constants::*;

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

#[derive(Debug, Default)]
pub struct App {
    pub should_exit: bool,
    pub pomo: Pomo,
    pub session_name: TextArea<'static>,
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();

        app.session_name.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" Session Name ").centered()),
        );
        app
    }
}
