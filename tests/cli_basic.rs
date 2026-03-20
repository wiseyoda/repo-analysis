use assert_cmd::Command;

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
