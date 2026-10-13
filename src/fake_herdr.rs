use std::cell::RefCell;

use crate::herdr::Herdr;

pub struct FakeHerdr {
    runs_nvim: bool,
    pub probed: RefCell<Vec<String>>,
    pub ran: RefCell<Vec<Vec<String>>>,
}

impl FakeHerdr {
    pub fn running_nvim(runs_nvim: bool) -> Self {
        Self {
            runs_nvim,
            probed: RefCell::default(),
            ran: RefCell::default(),
        }
    }
}

impl Herdr for FakeHerdr {
    fn foreground_is_nvim(&self, pane: &str) -> bool {
        self.probed.borrow_mut().push(pane.to_string());
        self.runs_nvim
    }

    fn run(&self, argv: &[&str]) {
        self.ran
            .borrow_mut()
            .push(argv.iter().map(|arg| arg.to_string()).collect());
    }
}
