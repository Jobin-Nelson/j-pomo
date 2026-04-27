use std::path::PathBuf;

use crate::{Error, Result};

pub fn create_state_file() -> Result<PathBuf> {
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

    if state_file.exists() {
        Err(Error::AnotherInstanceInUse)
    } else {
        Ok(state_file)
    }
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_if_state_file_present() -> Result<()> {
        // -- Setup & Fixtures
        let state_file = create_state_file()?;
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&state_file)?;

        // -- Exec
        let result = create_state_file();
        std::fs::remove_file(state_file)?;

        // -- Check
        println!("{result:?}");
        assert!(matches!(result, Err(Error::AnotherInstanceInUse)));

        Ok(())
    }
}

// endregion: --- Tests
