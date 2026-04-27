use std::path::PathBuf;

pub fn get_state_file() -> PathBuf {
    let state_file = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".local/state")
        })
        .join("pomodoro/status.txt");

    if let Some(parent) = state_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    state_file
}

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
