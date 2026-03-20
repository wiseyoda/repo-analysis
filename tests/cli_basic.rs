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
fn exits_successfully_without_arguments() {
    repostat().assert().success();
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
    repostat().assert().success();
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
