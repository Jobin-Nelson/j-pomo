# Pomodoro

<!--toc:start-->

- [Pomodoro](#pomodoro)
  - [About](#about)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Usage](#usage)
  - [Development Contribution](#development-contribution)
  - [License](#license)
  <!--toc:end-->

│ ⏱️ Terminal-based Pomodoro Timer using Ratatui (https://ratatui.rs/) for TUI interface

## About

A cross-platform terminal-based Pomodoro timer built with Ratatui for enhanced text user interface. Implements a 25-minute work
session with 5-minute break intervals following standard Pomodoro techniques.

## Features

- 25/5 Pomodoro Timer
- Cross-platform support (Windows/macOS/Linux)
- Current state is written to `${XDG_STATE_HOME}/pomodoro/status.txt` which can be read by other applications (waybar, tmux...)
- Alerts user after each session is completed
- Keybinds
  - `p`: toggle pause/play
  - `n`: next session (FOCUS/BREAK)
  - `N`: previous session (FOCUS/BREAK)
  - `r`: reset current session
  - `q`: quit

## Getting Started

```bash
cargo install --locked j-pomo
```

### Usage

```bash
pomo
```

## Development Contribution

Contributions are welcome! Please follow these guidelines:

1.  Fork the repository
2.  Create a feature branch (git checkout -b feature/xyz)
3.  Implement changes and update documentation
4.  Submit pull requests

Best practices:

- Update tests for new features
- Follow Rustfmt/Cargo.toml conventions
- Use semantic versioning for updates

## License

Copyright (c) jobin_nelson <jobinnelson369@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)
