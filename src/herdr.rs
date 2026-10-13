use crate::decision::{Action, decide};
use crate::direction::Direction;

pub trait Herdr {
    fn foreground_is_nvim(&self, pane: &str) -> bool;
    fn run(&self, argv: &[&str]);
}

pub fn argv(action: &Action) -> Vec<&str> {
    match action {
        Action::SendKeys { pane, direction } => vec!["pane", "send-keys", pane, direction.chord()],
        Action::Focus { pane, direction } => vec![
            "pane",
            "focus",
            "--direction",
            direction.as_str(),
            "--pane",
            pane,
        ],
        Action::FocusCurrent { direction } => {
            vec![
                "pane",
                "focus",
                "--direction",
                direction.as_str(),
                "--current",
            ]
        }
    }
}

pub fn navigate(herdr: &impl Herdr, pane: Option<&str>, direction: Direction) {
    let is_nvim = pane.is_some_and(|pane| herdr.foreground_is_nvim(pane));
    herdr.run(&argv(&decide(pane, is_nvim, direction)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_herdr::FakeHerdr;

    #[test]
    fn sending_keys_names_the_pane_and_the_chord() {
        let action = Action::SendKeys {
            pane: "wW:p8".into(),
            direction: Direction::Left,
        };
        assert_eq!(argv(&action), ["pane", "send-keys", "wW:p8", "ctrl+h"]);
    }

    #[test]
    fn focusing_from_a_pane_names_the_direction_and_the_pane() {
        let action = Action::Focus {
            pane: "wW:p8".into(),
            direction: Direction::Down,
        };
        assert_eq!(
            argv(&action),
            ["pane", "focus", "--direction", "down", "--pane", "wW:p8"]
        );
    }

    #[test]
    fn focusing_with_no_pane_moves_the_current_one() {
        let action = Action::FocusCurrent {
            direction: Direction::Up,
        };
        assert_eq!(
            argv(&action),
            ["pane", "focus", "--direction", "up", "--current"]
        );
    }

    #[test]
    fn a_press_with_no_pane_moves_the_current_pane_without_probing() {
        let herdr = FakeHerdr::running_nvim(true);
        navigate(&herdr, None, Direction::Right);
        assert!(herdr.probed.borrow().is_empty());
        assert_eq!(
            *herdr.ran.borrow(),
            [["pane", "focus", "--direction", "right", "--current"]]
        );
    }

    #[test]
    fn neovim_is_sent_the_chord_for_each_direction() {
        for (direction, chord) in [
            (Direction::Left, "ctrl+h"),
            (Direction::Down, "ctrl+j"),
            (Direction::Up, "ctrl+k"),
            (Direction::Right, "ctrl+l"),
        ] {
            let herdr = FakeHerdr::running_nvim(true);
            navigate(&herdr, Some("wW:p8"), direction);
            assert_eq!(*herdr.probed.borrow(), ["wW:p8"]);
            assert_eq!(*herdr.ran.borrow(), [["pane", "send-keys", "wW:p8", chord]]);
        }
    }

    #[test]
    fn a_pane_running_anything_else_is_moved() {
        let herdr = FakeHerdr::running_nvim(false);
        navigate(&herdr, Some("wW:p8"), Direction::Up);
        assert_eq!(*herdr.probed.borrow(), ["wW:p8"]);
        assert_eq!(
            *herdr.ran.borrow(),
            [["pane", "focus", "--direction", "up", "--pane", "wW:p8"]]
        );
    }
}
