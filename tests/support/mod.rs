use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::sync::atomic::{AtomicU64, Ordering};

const HERDR_DOUBLE: &str = include_str!("herdr-double.sh");

static NEXT: AtomicU64 = AtomicU64::new(0);

pub struct Fixture {
    root: PathBuf,
    pub command: Command,
}

pub struct Result {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
    pub calls: Vec<Vec<String>>,
}

impl Fixture {
    pub fn new(args: &[&str]) -> Self {
        let root = std::env::temp_dir().join(format!(
            "smart-nav-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let binary = root.join("herdr double");
        fs::write(&binary, HERDR_DOUBLE).unwrap();
        fs::set_permissions(&binary, Permissions::from_mode(0o700)).unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_herdr-smart-nav"));
        command
            .args(args)
            .env_clear()
            .current_dir(&root)
            .env("PATH", &root)
            .env("TEST_ROOT", &root)
            .env("HERDR_BIN_PATH", binary);
        Self { root, command }
    }

    pub fn run(mut self) -> Result {
        let output = self.command.output().unwrap();
        let calls = (1..)
            .map_while(|n| fs::read(self.root.join(format!("call-{n}"))).ok())
            .map(|bytes| {
                bytes[..bytes.len() - 1]
                    .split(|b| *b == 0)
                    .map(|arg| String::from_utf8(arg.to_vec()).unwrap())
                    .collect()
            })
            .collect();
        Result {
            status: output.status,
            stdout: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
            calls,
        }
    }
}
