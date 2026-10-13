use herdr_smart_nav_application::argv;
use herdr_smart_nav_domain::{Action, Direction};

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
