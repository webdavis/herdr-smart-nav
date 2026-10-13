use crate::direction::Direction;

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
