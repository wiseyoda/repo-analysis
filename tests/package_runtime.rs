use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::tempdir;

fn script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join("prepare-suite-runtime.sh")
}

fn run(binary: &Path, destination: &Path) -> std::process::Output {
    Command::new("sh")
        .arg(script())
        .arg("--binary")
        .arg(binary)
        .arg(destination)
        .output()
        .expect("runtime staging command should start")
}

#[test]
fn stages_only_the_closed_runtime_with_normalized_modes() {
    let fixture = tempdir().expect("fixture should be created");
    let binary = fixture.path().join("repostat-release");
    fs::write(&binary, b"fixture release bytes").expect("binary should be written");
    fs::set_permissions(&binary, fs::Permissions::from_mode(0o755))
        .expect("binary should be executable");
    let destination = fixture.path().join("runtime");

    let output = run(&binary, &destination);

    assert!(
        output.status.success(),
        "staging failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut files = vec![
        PathBuf::from("ai-mux.extension.json"),
        PathBuf::from("repostat"),
        PathBuf::from("schemas/repostat-metrics-v1.schema.json"),
    ];
    files.sort();
    let mut actual = walk_files(&destination);
    actual.sort();
    assert_eq!(actual, files);
    assert_eq!(
        fs::read(destination.join("repostat")).expect("binary should be readable"),
        b"fixture release bytes"
    );
    assert_eq!(mode(&destination.join("repostat")), 0o500);
    assert_eq!(mode(&destination.join("ai-mux.extension.json")), 0o400);
    assert_eq!(
        mode(&destination.join("schemas/repostat-metrics-v1.schema.json")),
        0o400
    );

    let repeated = run(&binary, &destination);
    assert!(!repeated.status.success());
    assert!(String::from_utf8_lossy(&repeated.stderr).contains("destination already exists"));
}

#[test]
fn rejects_missing_or_non_executable_binaries_without_publishing_a_root() {
    let fixture = tempdir().expect("fixture should be created");
    let missing = fixture.path().join("missing");
    let destination = fixture.path().join("runtime");

    let missing_output = run(&missing, &destination);

    assert!(!missing_output.status.success());
    assert!(String::from_utf8_lossy(&missing_output.stderr).contains("does not exist"));
    assert!(!destination.exists());

    fs::write(&missing, b"not executable").expect("fixture should be written");
    let non_executable_output = run(&missing, &destination);
    assert!(!non_executable_output.status.success());
    assert!(String::from_utf8_lossy(&non_executable_output.stderr).contains("not executable"));
    assert!(!destination.exists());
}

fn walk_files(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("directory should be readable") {
            let entry = entry.expect("entry should be readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(
                    path.strip_prefix(root)
                        .expect("path should be within root")
                        .to_path_buf(),
                );
            }
        }
    }
    files
}

fn mode(path: &Path) -> u32 {
    fs::metadata(path)
        .expect("path should exist")
        .permissions()
        .mode()
        & 0o777
}
