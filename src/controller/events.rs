use crossterm::event;

use crate::models::{PomoProgress, RemainingSecs};

pub enum Event {
    Input(event::KeyEvent),
    Resize,
    PomoUpdate(RemainingSecs, PomoProgress),
    PomoDone,
}

pub enum PomoCommand {
    Start(u32),
    Pause,
    Resume,
    Quit,
}
