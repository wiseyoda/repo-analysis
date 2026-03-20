use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn prints_help_without_error() {
    Command::cargo_bin("repostat")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn prints_version_without_error() {
    Command::cargo_bin("repostat")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn exits_successfully_without_arguments() {
    Command::cargo_bin("repostat").unwrap().assert().success();
}

#[test]
fn accepts_valid_directory_path() {
    let dir = TempDir::new().unwrap();
    Command::cargo_bin("repostat")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success();
}

#[test]
fn errors_on_nonexistent_path() {
    Command::cargo_bin("repostat")
        .unwrap()
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

    Command::cargo_bin("repostat")
        .unwrap()
        .arg(&file_path)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("path is not a directory"));
}

#[test]
fn defaults_to_current_directory() {
    // Running without args should succeed (uses cwd)
    Command::cargo_bin("repostat").unwrap().assert().success();
}
