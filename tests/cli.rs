#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new(compiler: &str, agent: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "sentinel-cli-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        for (name, body) in [("compiler", compiler), ("opencode", agent)] {
            let script = path.join(name);
            fs::write(&script, format!("#!/bin/sh\n{body}\n")).unwrap();
            fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        }
        Self(path)
    }

    fn run(&self, options: &[&str], compiler_args: &[&str]) -> Output {
        let mut paths = vec![self.0.clone()];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        Command::new(env!("CARGO_BIN_EXE_ai-sentinel"))
            .args(options)
            .arg(self.0.join("compiler"))
            .args(compiler_args)
            .current_dir(&self.0)
            .env("PATH", std::env::join_paths(paths).unwrap())
            .output()
            .unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn successful_stderr_does_not_trigger_default_repair() {
    let fixture = Fixture::new(
        "echo 'warning: harmless' >&2\nexit 0",
        "touch agent-called\nexit 0",
    );
    let result = fixture.run(&[], &[]);
    assert!(result.status.success());
    assert!(!fixture.0.join("agent-called").exists());
    assert!(String::from_utf8_lossy(&result.stderr).contains("warning: harmless"));
}

#[test]
fn failed_build_with_only_stdout_triggers_repair_and_verification() {
    let fixture = Fixture::new(
        "printf '%s\\n' \"$@\" > compiler-args\nif [ -f repaired ]; then echo built; exit 0; fi\necho 'compile error'\nexit 1",
        "printf '%s\\n' \"$@\" > agent-args\ntouch repaired\nexit 0",
    );
    let result = fixture.run(&[], &["-Wall", "file with spaces.cpp"]);
    assert!(result.status.success(), "{:?}", result);
    assert!(String::from_utf8_lossy(&result.stderr).contains("build verified successfully"));
    let prompt = fs::read_to_string(fixture.0.join("agent-args")).unwrap();
    assert!(prompt.contains("compile error"));
    assert!(prompt.contains("file with spaces.cpp"));
    assert!(!prompt.contains("cannot locate skill"));
    assert_eq!(
        fs::read_to_string(fixture.0.join("compiler-args")).unwrap(),
        "-Wall\nfile with spaces.cpp\n"
    );
}

#[test]
fn silent_failure_still_triggers_repair() {
    let fixture = Fixture::new("test -f repaired", "touch repaired\nexit 0");
    assert!(fixture.run(&[], &[]).status.success());
    assert!(fixture.0.join("repaired").exists());
}

#[test]
fn successful_agent_cannot_mask_a_failing_build() {
    let fixture = Fixture::new("exit 1", "exit 0");
    let result = fixture.run(&[], &[]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("S006"));
}

#[test]
fn agent_failure_is_reported_without_success() {
    let fixture = Fixture::new("exit 1", "echo 'agent failed' >&2\nexit 2");
    let result = fixture.run(&[], &[]);
    assert!(!result.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("S002"));
    assert!(stderr.contains("agent failed"));
    assert!(!stderr.contains("build verified successfully"));
}

#[test]
fn warning_mode_can_request_repair_after_successful_build() {
    let fixture = Fixture::new("echo warning >&2\nexit 0", "touch agent-called\nexit 0");
    assert!(fixture.run(&["--level", "warning"], &[]).status.success());
    assert!(fixture.0.join("agent-called").exists());
}
