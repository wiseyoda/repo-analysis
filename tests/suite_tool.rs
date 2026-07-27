use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

fn create_fixture(root: &Path) {
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("src/lib.rs"),
        "pub fn answer(value: bool) -> usize {\n    if value { 42 } else { 0 }\n}\n",
    )
    .unwrap();
    fs::write(
        root.join("README.md"),
        "# Fixture\n\n## Install\n\n## Usage\n\n## License\n",
    )
    .unwrap();
}

fn provider_traps(root: &Path, marker: &Path) -> PathBuf {
    let bin = root.join("trap-bin");
    fs::create_dir_all(&bin).unwrap();
    for name in ["agy", "claude", "codex", "cursor-agent", "grok", "lvl"] {
        let path = bin.join(name);
        fs::write(
            &path,
            format!(
                "#!/bin/sh\nprintf '%s' invoked > '{}'\nexit 91\n",
                marker.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    bin
}

fn test_command(home: &Path, trap_bin: &Path) -> Command {
    let mut command = Command::cargo_bin("repostat").unwrap();
    let inherited_path = std::env::var("PATH").unwrap_or_default();
    command
        .env("HOME", home)
        .env("PATH", format!("{}:{inherited_path}", trap_bin.display()));
    command
}

fn git(root: &Path, arguments: &[&str]) -> Vec<u8> {
    let output = std::process::Command::new("git")
        .args(arguments)
        .current_dir(root)
        .env("GIT_AUTHOR_DATE", "2020-01-02T03:04:05Z")
        .env("GIT_COMMITTER_DATE", "2020-01-02T03:04:05Z")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn tree_fingerprint(root: &Path) -> BTreeMap<String, (u32, Vec<u8>)> {
    fn visit(root: &Path, current: &Path, entries: &mut BTreeMap<String, (u32, Vec<u8>)>) {
        let mut children: Vec<_> = fs::read_dir(current)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        children.sort();
        for path in children {
            let relative = path.strip_prefix(root).unwrap().display().to_string();
            let metadata = fs::symlink_metadata(&path).unwrap();
            if metadata.is_dir() {
                entries.insert(relative, (metadata.permissions().mode(), Vec::new()));
                visit(root, &path, entries);
            } else {
                entries.insert(
                    relative,
                    (metadata.permissions().mode(), fs::read(&path).unwrap()),
                );
            }
        }
    }

    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

#[test]
fn default_scan_is_token_free_and_no_write() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);
    let before = tree_fingerprint(&target);

    test_command(&home, &traps).arg(&target).assert().success();

    assert_eq!(tree_fingerprint(&target), before);
    assert!(!home.join(".repostat").exists());
    assert!(!marker.exists(), "a provider/model process was invoked");
}

#[test]
fn structured_json_is_stable_closed_and_no_write() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);
    let before = tree_fingerprint(&target);

    let first = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();
    let second = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();

    assert!(first.status.success());
    assert_eq!(first.stdout, second.stdout);
    let value: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(value["schemaVersion"], "1");
    assert_eq!(value["artifactType"], "repostat.metrics.v1");
    assert_eq!(
        value["source"]["canonicalRoot"],
        fs::canonicalize(&target).unwrap().display().to_string()
    );
    assert!(value.get("timestamp").is_none());
    assert!(value.get("aiAnalysis").is_none());
    assert!(value["documentation"].get("fileCount").is_some());
    assert!(value["documentation"].get("file_count").is_none());
    assert_eq!(tree_fingerprint(&target), before);
    assert!(!home.join(".repostat").exists());
    assert!(!marker.exists());
}

#[test]
fn extension_and_standalone_results_are_byte_identical() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);
    let before = tree_fingerprint(&target);

    let standalone = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();
    let extension = test_command(&home, &traps)
        .arg("extension")
        .arg(&target)
        .output()
        .unwrap();

    assert!(standalone.status.success());
    assert!(extension.status.success());
    assert_eq!(extension.stderr, b"");
    assert_eq!(extension.stdout, standalone.stdout);
    assert_eq!(tree_fingerprint(&target), before);
    assert!(!home.join(".repostat").exists());
    assert!(!marker.exists());
}

#[test]
fn save_is_the_explicit_history_write() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);

    test_command(&home, &traps)
        .arg("--save")
        .arg(&target)
        .assert()
        .success();

    let snapshots = target.join(".repostat/snapshots");
    assert!(snapshots.is_dir());
    assert!(fs::read_dir(snapshots).unwrap().next().is_some());
    assert!(home.join(".repostat/repos.json").is_file());
    assert!(!marker.exists());
}

#[test]
fn saved_history_does_not_change_future_metrics() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);

    let before = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();
    test_command(&home, &traps)
        .arg("--save")
        .arg(&target)
        .assert()
        .success();
    let after = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();

    assert!(before.status.success());
    assert!(after.status.success());
    assert_eq!(after.stdout, before.stdout);
}

#[test]
fn target_git_sha_comes_from_the_analyzed_repository() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    git(&target, &["init", "-b", "main"]);
    git(&target, &["config", "user.name", "Fixture"]);
    git(
        &target,
        &["config", "user.email", "fixture@example.invalid"],
    );
    git(&target, &["add", "."]);
    git(&target, &["commit", "-m", "test: fixture"]);
    let expected_sha = String::from_utf8(git(&target, &["rev-parse", "HEAD"]))
        .unwrap()
        .trim()
        .to_string();
    let traps = provider_traps(workspace.path(), &marker);

    let output = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();

    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["source"]["gitSha"], expected_sha);
}

#[test]
fn user_global_gitignore_cannot_change_metrics() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    git(&target, &["init", "-b", "main"]);
    let global_ignore = home.join("global-ignore");
    fs::write(&global_ignore, "*.rs\n").unwrap();
    fs::write(
        home.join(".gitconfig"),
        format!("[core]\n\texcludesfile = {}\n", global_ignore.display()),
    )
    .unwrap();
    let traps = provider_traps(workspace.path(), &marker);

    let output = test_command(&home, &traps)
        .args(["--json", "--no-write"])
        .arg(&target)
        .output()
        .unwrap();

    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["byLanguage"]["Rust"]["files"], 1);
}

#[test]
fn no_write_rejects_write_flags() {
    let workspace = TempDir::new().unwrap();
    let target = workspace.path().join("target");
    let home = workspace.path().join("home");
    let marker = workspace.path().join("provider-invoked");
    fs::create_dir_all(&home).unwrap();
    create_fixture(&target);
    let traps = provider_traps(workspace.path(), &marker);

    test_command(&home, &traps)
        .args(["--no-write", "--save"])
        .arg(&target)
        .assert()
        .failure();
    test_command(&home, &traps)
        .args(["--no-write", "--html"])
        .arg(&target)
        .assert()
        .failure();
    assert!(!marker.exists());
}

#[test]
fn suite_manifest_and_result_schema_are_closed() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../ai-mux.extension.json")).unwrap();
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/repostat-metrics-v1.schema.json")).unwrap();

    assert_eq!(manifest["id"], "ai-mux.repostat");
    assert_eq!(manifest["commands"][0], "repostat.scan");
    assert_eq!(manifest["artifactTypes"][0], "repostat.metrics.v1");
    assert_eq!(manifest["engineCapabilities"], serde_json::json!([]));
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(
        schema["properties"]["artifactType"]["const"],
        "repostat.metrics.v1"
    );
    assert!(
        schema["$defs"]["dependencies"]["properties"]
            .get("manifestCount")
            .is_some()
    );
    assert!(
        schema["$defs"]["riskHotspot"]["properties"]
            .get("churnCount")
            .is_some()
    );
}

#[test]
fn distribution_metadata_uses_the_suite_repository() {
    for content in [
        include_str!("../Cargo.toml"),
        include_str!("../README.md"),
        include_str!("../Formula/repostat.rb"),
    ] {
        assert!(content.contains("https://github.com/wiseyoda/ai-mux-repostat"));
        assert!(!content.contains("https://github.com/wiseyoda/repo-analysis"));
    }
}
