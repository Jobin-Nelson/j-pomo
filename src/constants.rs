use indoc::indoc;

pub const FOCUS_MINS: u32 = 25;
pub const BREAK_MINS: u32 = 5;
pub const LONG_BREAK_MINS: u32 = 15;

pub const LOGO: &str = indoc! {"
    █▀█ █▀█ █▀▄▀█ █▀█
    █▀▀ █▄█ █ ▀ █ █▄█
    "
};

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consts() {
        // -- Setup & Fixtures

        // -- Exec

        // -- Check
        let expected_focus_mins = 25;
        let expected_break_mins = 5;

        assert_eq!(FOCUS_MINS, expected_focus_mins);
        assert_eq!(BREAK_MINS, expected_break_mins);
    }
}

// endregion: --- Tests
