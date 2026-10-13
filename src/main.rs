use std::env;

use direction::Direction;
use herdr::navigate;
use herdr_cli::HerdrCli;

mod decision;
mod direction;
#[cfg(test)]
mod fake_herdr;
mod herdr;
mod herdr_cli;
mod pane;
mod process_info;

const DEFAULT_HERDR: &str = "herdr";

const USAGE: &str = "usage: herdr-smart-nav left|down|up|right";

fn main() {
    let word = env::args_os()
        .nth(1)
        .and_then(|argument| argument.into_string().ok())
        .unwrap_or_default();
    let Some(direction) = Direction::parse(&word) else {
        eprintln!("{USAGE}");
        std::process::exit(2);
    };
    let pane = pane::resolve_pane(
        env::var("HERDR_PANE_ID").ok(),
        env::var("HERDR_ACTIVE_PANE_ID").ok(),
    );
    let herdr =
        HerdrCli::new(env::var("HERDR_BIN_PATH").unwrap_or_else(|_| DEFAULT_HERDR.to_string()));
    navigate(&herdr, pane.as_deref(), direction);
}
