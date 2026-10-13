use herdr_smart_nav_domain::{Direction, decide};

mod herdr;

pub use herdr::{Herdr, argv};

pub fn navigate(herdr: &impl Herdr, pane: Option<&str>, direction: Direction) {
    let is_nvim = pane.is_some_and(|pane| herdr.foreground_is_nvim(pane));
    herdr.run(&argv(&decide(pane, is_nvim, direction)));
}
