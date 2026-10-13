mod support;

use herdr_smart_nav_application::navigate;
use herdr_smart_nav_domain::Direction;
use support::FakeHerdr;

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
