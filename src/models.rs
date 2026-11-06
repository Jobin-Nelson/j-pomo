use std::fmt::Display;
use std::path::PathBuf;

use crate::{Error, Result, config::create_state_file, constants::*};

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
    pub platform: Platform,
}

#[derive(Debug)]
pub enum Platform {
    Windows,
    Wsl,
    Linux,
    MacOs,
}

fn is_wsl() -> bool {
    if let Ok(version) = std::fs::read_to_string("/proc/version") {
        let version_lower = version.to_lowercase();
        if version_lower.contains("microsoft") || version_lower.contains("wsl") {
            return true;
        }
    }

    false
}

fn get_platform() -> Result<Platform> {
    if cfg!(target_os = "windows") {
        return Ok(Platform::Windows);
    }
    if cfg!(target_os = "macos") {
        return Ok(Platform::MacOs);
    }
    if cfg!(target_os = "linux") {
        if is_wsl() {
            return Ok(Platform::Wsl);
        } else {
            return Ok(Platform::Linux);
        }
    }

    Err(Error::UnknownEnvironment)
}

impl App {
    pub fn new() -> Result<Self> {
        let state_file = StateFileGuard {
            path: create_state_file()?,
        };

        let platform = get_platform()?;

        let mut app = App {
            state_file,
            should_exit: Default::default(),
            pomo: Default::default(),
            session_name: Default::default(),
            mode: Default::default(),
            platform,
        };

        app.session_name.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" Session Name ").centered()),
        );
        Ok(app)
    }
}
