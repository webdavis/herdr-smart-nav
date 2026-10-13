use std::process::Command;

use crate::herdr::Herdr;
use crate::process_info::is_nvim_foreground;

pub struct HerdrCli {
    binary: String,
}

impl HerdrCli {
    pub fn new(binary: String) -> Self {
        Self { binary }
    }
}

impl Herdr for HerdrCli {
    fn foreground_is_nvim(&self, pane: &str) -> bool {
        Command::new(&self.binary)
            .args(["pane", "process-info", "--pane", pane])
            .output()
            .is_ok_and(|answered| is_nvim_foreground(&String::from_utf8_lossy(&answered.stdout)))
    }

    fn run(&self, argv: &[&str]) {
        let _ = Command::new(&self.binary).args(argv).status();
    }
}
