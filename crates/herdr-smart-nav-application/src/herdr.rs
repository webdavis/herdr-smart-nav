use herdr_smart_nav_domain::Action;

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
