use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Create a repostat command with AI disabled for fast tests.
fn repostat() -> Command {
    let mut cmd = Command::cargo_bin("repostat").unwrap();
    cmd.env("REPOSTAT_SKIP_AI", "1");
    cmd
}

#[test]
fn prints_help_without_error() {
    repostat().arg("--help").assert().success();
}

#[test]
fn prints_version_without_error() {
    repostat().arg("--version").assert().success();
}

#[test]
fn exits_without_tool_error() {
    // May exit 0 (healthy), 10 (warning), or 20 (critical) — all valid.
    // Exit 1 or 2 would be a tool error.
    let output = repostat().output().expect("failed to run");
    let code = output.status.code().unwrap_or(1);
    assert!(
        code == 0 || code == 10 || code == 20,
        "expected health exit code, got {code}",
    );
}

#[test]
fn accepts_valid_directory_path() {
    let dir = TempDir::new().unwrap();
    repostat().arg(dir.path()).assert().success();
}

#[test]
fn errors_on_nonexistent_path() {
    repostat()
        .arg("/tmp/repostat-nonexistent-path-abc123")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("path does not exist"));
}

#[test]
fn errors_when_path_is_a_file() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("not-a-dir.txt");
    std::fs::write(&file_path, "hello").unwrap();

    repostat()
        .arg(&file_path)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("path is not a directory"));
}

#[test]
fn defaults_to_current_directory() {
    // Analyzing current dir may trigger health exit codes (10/20)
    let output = repostat().output().expect("failed to run");
    let code = output.status.code().unwrap_or(1);
    assert!(
        code == 0 || code == 10 || code == 20,
        "expected health exit code, got {code}",
    );
}

#[test]
fn init_creates_config_file() {
    let dir = TempDir::new().unwrap();
    repostat()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();
    assert!(dir.path().join(".repostat.toml").exists());
    let content = std::fs::read_to_string(dir.path().join(".repostat.toml")).unwrap();
    assert!(content.contains("[health]"));
    assert!(content.contains("warn_complexity"));
}

#[test]
fn init_errors_on_existing_config() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join(".repostat.toml"), "existing").unwrap();
    repostat()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn init_force_overwrites() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join(".repostat.toml"), "old").unwrap();
    repostat()
        .args(["init", "--force"])
        .current_dir(dir.path())
        .assert()
        .success();
    let content = std::fs::read_to_string(dir.path().join(".repostat.toml")).unwrap();
    assert!(content.contains("[health]"));
}

#[test]
fn diff_shows_changed_files() {
    // Run diff on the current repo (which is a git repo)
    let output = repostat()
        .args(["diff", "HEAD~1"])
        .output()
        .expect("failed to run");
    let code = output.status.code().unwrap_or(1);
    // Should succeed (0) or show health codes (10/20) or show "no files changed"
    assert!(
        code == 0 || code == 10 || code == 20,
        "diff should not fail with tool error, got {code}",
    );
}

#[test]
fn diff_errors_on_non_git_dir() {
    let dir = TempDir::new().unwrap();
    repostat()
        .args(["diff", "HEAD~1", dir.path().to_str().unwrap()])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn warns_on_empty_directory() {
    let dir = TempDir::new().unwrap();
    repostat()
        .arg(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "no source files found after filtering",
        ));
}

#[test]
fn verbose_shows_timing() {
    let dir = TempDir::new().unwrap();
    repostat()
        .args(["--verbose", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("scanner:").and(predicate::str::contains("total:")));
}

#[test]
fn no_timing_without_verbose() {
    let dir = TempDir::new().unwrap();
    repostat()
        .arg(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("scanner:").not());
}
