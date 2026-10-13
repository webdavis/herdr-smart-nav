#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

impl Direction {
    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "left" => Some(Direction::Left),
            "down" => Some(Direction::Down),
            "up" => Some(Direction::Up),
            "right" => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn chord(self) -> &'static str {
        match self {
            Direction::Left => "ctrl+h",
            Direction::Down => "ctrl+j",
            Direction::Up => "ctrl+k",
            Direction::Right => "ctrl+l",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Direction::Left => "left",
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::Right => "right",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    SendKeys { pane: String, direction: Direction },
    Focus { pane: String, direction: Direction },
    FocusCurrent { direction: Direction },
}

pub fn decide(pane: Option<&str>, is_nvim: bool, direction: Direction) -> Action {
    match pane {
        Some(pane) if is_nvim => Action::SendKeys {
            pane: pane.to_string(),
            direction,
        },
        Some(pane) => Action::Focus {
            pane: pane.to_string(),
            direction,
        },
        None => Action::FocusCurrent { direction },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_direction_carries_its_own_chord_and_its_own_word() {
        for (word, direction, chord) in [
            ("left", Direction::Left, "ctrl+h"),
            ("down", Direction::Down, "ctrl+j"),
            ("up", Direction::Up, "ctrl+k"),
            ("right", Direction::Right, "ctrl+l"),
        ] {
            assert_eq!(Direction::parse(word), Some(direction));
            assert_eq!(direction.chord(), chord);
            assert_eq!(direction.as_str(), word, "herdr's own spelling");
        }
    }

    #[test]
    fn a_word_this_does_not_serve_is_refused() {
        assert_eq!(Direction::parse("sideways"), None);
        assert_eq!(Direction::parse(""), None);
        assert_eq!(Direction::parse("Left"), None);
    }

    #[test]
    fn a_pane_running_neovim_is_sent_the_chord() {
        assert_eq!(
            decide(Some("wW:p8"), true, Direction::Left),
            Action::SendKeys {
                pane: "wW:p8".into(),
                direction: Direction::Left,
            }
        );
    }

    #[test]
    fn a_pane_running_anything_else_moves_the_pane() {
        assert_eq!(
            decide(Some("wW:p8"), false, Direction::Left),
            Action::Focus {
                pane: "wW:p8".into(),
                direction: Direction::Left,
            }
        );
    }

    #[test]
    fn a_press_with_no_pane_moves_the_current_one() {
        assert_eq!(
            decide(None, false, Direction::Left),
            Action::FocusCurrent {
                direction: Direction::Left,
            }
        );
    }
}
