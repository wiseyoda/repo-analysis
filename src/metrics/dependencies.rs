//! Dependency manifest parsing for multiple language ecosystems.

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

/// Known manifest filenames.
const MANIFEST_NAMES: &[&str] = &[
    "Cargo.toml",
    "package.json",
    "requirements.txt",
    "Pipfile",
    "go.mod",
    "Gemfile",
    "pom.xml",
    "build.gradle",
    "Package.swift",
];

/// Directories to skip when searching for manifests.
const EXCLUDE_DIRS: &[&str] = &[
    "node_modules",
    "vendor",
    "build",
    "dist",
    ".next",
    "Pods",
    "target",
    ".git",
    "__pycache__",
    ".venv",
    "venv",
];

/// Type of dependency manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManifestType {
    /// Rust (Cargo.toml)
    Cargo,
    /// Node.js (package.json)
    Npm,
    /// Python (requirements.txt, Pipfile)
    Python,
    /// Go (go.mod)
    Go,
    /// Swift (Package.swift)
    Swift,
    /// Ruby (Gemfile)
    Ruby,
    /// Java Maven (pom.xml)
    Maven,
    /// Java/Kotlin Gradle (build.gradle)
    Gradle,
}

/// Parsed manifest information.
#[derive(Debug, Clone)]
pub(crate) struct ManifestInfo {
    /// Type of manifest.
    pub(crate) manifest_type: ManifestType,
    /// Path to the manifest file.
    pub(crate) file_path: PathBuf,
    /// Direct dependency names.
    pub(crate) direct_deps: Vec<String>,
}

/// Summary of all dependency manifests in a project.
#[derive(Debug, Clone, Default)]
pub(crate) struct DependencySummary {
    /// All parsed manifests.
    pub(crate) manifests: Vec<ManifestInfo>,
    /// Total direct dependencies across all manifests.
    pub(crate) total_direct: usize,
    /// Transitive dependency count (from lock files, if available).
    pub(crate) total_transitive: Option<usize>,
}

/// Known lock file names.
const LOCK_FILE_NAMES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "poetry.lock",
    "Pipfile.lock",
    "go.sum",
    "Gemfile.lock",
];

/// Scan a directory for manifests and summarize dependency counts.
pub(crate) fn summarize_dependencies(dir: &Path) -> DependencySummary {
    let paths = find_manifests(dir);
    let manifests: Vec<ManifestInfo> = paths.iter().filter_map(|p| parse_manifest(p)).collect();
    let total_direct = manifests.iter().map(|m| m.direct_deps.len()).sum();
    let total_transitive = count_transitive_deps(dir);

    DependencySummary {
        manifests,
        total_direct,
        total_transitive,
    }
}

/// Count transitive dependencies from lock files in the directory.
fn count_transitive_deps(dir: &Path) -> Option<usize> {
    let mut total = 0;
    let mut found_any = false;

    for name in LOCK_FILE_NAMES {
        let path = dir.join(name);
        if path.exists() {
            if let Some(count) = count_lock_file_entries(&path, name) {
                total += count;
                found_any = true;
            }
        }
    }

    if found_any {
        Some(total)
    } else {
        None
    }
}

/// Count entries in a specific lock file.
fn count_lock_file_entries(path: &Path, name: &str) -> Option<usize> {
    let content = std::fs::read_to_string(path).ok()?;

    match name {
        "Cargo.lock" => Some(count_cargo_lock(&content)),
        "package-lock.json" => Some(count_package_lock(&content)),
        "yarn.lock" => Some(count_yarn_lock(&content)),
        "go.sum" => Some(count_go_sum(&content)),
        "Gemfile.lock" => Some(count_gemfile_lock(&content)),
        "poetry.lock" => Some(count_poetry_lock(&content)),
        _ => None,
    }
}

/// Count packages in Cargo.lock (each [[package]] block).
fn count_cargo_lock(content: &str) -> usize {
    content
        .lines()
        .filter(|l| l.trim() == "[[package]]")
        .count()
}

/// Count packages in package-lock.json (keys in "packages" object).
fn count_package_lock(content: &str) -> usize {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return 0;
    };
    // v3 format uses "packages", v1/v2 uses "dependencies"
    if let Some(pkgs) = value.get("packages").and_then(|v| v.as_object()) {
        pkgs.len().saturating_sub(1) // subtract root ""
    } else if let Some(deps) = value.get("dependencies").and_then(|v| v.as_object()) {
        deps.len()
    } else {
        0
    }
}

/// Count packages in yarn.lock (each unindented line with @ or quotes).
fn count_yarn_lock(content: &str) -> usize {
    content
        .lines()
        .filter(|l| {
            !l.is_empty()
                && !l.starts_with(' ')
                && !l.starts_with('#')
                && !l.starts_with("__metadata")
        })
        .count()
}

/// Count unique modules in go.sum (each unique module path).
fn count_go_sum(content: &str) -> usize {
    let modules: std::collections::HashSet<&str> = content
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .collect();
    modules.len()
}

/// Count gems in Gemfile.lock (specs section entries).
fn count_gemfile_lock(content: &str) -> usize {
    let mut in_specs = false;
    let mut count = 0;

    for line in content.lines() {
        if line.trim() == "specs:" {
            in_specs = true;
            continue;
        }
        if in_specs {
            if line.starts_with("    ") && !line.starts_with("      ") {
                count += 1;
            } else if !line.starts_with(' ') {
                in_specs = false;
            }
        }
    }

    count
}

/// Count packages in poetry.lock (each [[package]] block).
fn count_poetry_lock(content: &str) -> usize {
    content
        .lines()
        .filter(|l| l.trim() == "[[package]]")
        .count()
}

/// Find manifest files in a directory, skipping excluded directories.
pub(crate) fn find_manifests(dir: &Path) -> Vec<PathBuf> {
    let mut manifests = Vec::new();

    for entry in WalkBuilder::new(dir)
        .hidden(false)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !EXCLUDE_DIRS.iter().any(|&d| name == d)
        })
        .build()
        .filter_map(|e| e.ok())
    {
        if let Some(name) = entry.file_name().to_str() {
            if MANIFEST_NAMES.contains(&name) {
                manifests.push(entry.into_path());
            }
        }
    }

    manifests.sort();
    manifests
}

/// Parse a manifest file and extract dependency names.
///
/// Returns `None` if the file is not a recognized manifest type.
pub(crate) fn parse_manifest(path: &Path) -> Option<ManifestInfo> {
    let filename = path.file_name()?.to_str()?;
    let content = std::fs::read_to_string(path).ok()?;

    let (manifest_type, deps) = match filename {
        "Cargo.toml" => (ManifestType::Cargo, parse_cargo(&content)),
        "package.json" => (ManifestType::Npm, parse_package_json(&content)),
        "requirements.txt" => (ManifestType::Python, parse_requirements(&content)),
        "Pipfile" => (ManifestType::Python, parse_pipfile(&content)),
        "go.mod" => (ManifestType::Go, parse_go_mod(&content)),
        "Gemfile" => (ManifestType::Ruby, parse_gemfile(&content)),
        "pom.xml" => (ManifestType::Maven, parse_pom_xml(&content)),
        "build.gradle" => (ManifestType::Gradle, parse_gradle(&content)),
        "Package.swift" => (ManifestType::Swift, Vec::new()),
        _ => return None,
    };

    Some(ManifestInfo {
        manifest_type,
        file_path: path.to_path_buf(),
        direct_deps: deps,
    })
}

/// Parse Cargo.toml: extract keys from [dependencies], [dev-dependencies], [build-dependencies].
fn parse_cargo(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_deps_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            in_deps_section = trimmed == "[dependencies]"
                || trimmed == "[dev-dependencies]"
                || trimmed == "[build-dependencies]";
            continue;
        }

        if in_deps_section {
            if let Some(name) = trimmed.split('=').next() {
                let name = name.trim();
                if !name.is_empty() && !name.starts_with('#') {
                    deps.push(name.to_string());
                }
            }
        }
    }

    deps
}

/// Parse package.json: extract keys from dependencies and devDependencies.
fn parse_package_json(content: &str) -> Vec<String> {
    let mut deps = Vec::new();

    let Ok(value) = serde_json::from_str::<serde_json::Value>(content) else {
        return deps;
    };

    for section in ["dependencies", "devDependencies"] {
        if let Some(obj) = value.get(section).and_then(|v| v.as_object()) {
            for key in obj.keys() {
                deps.push(key.clone());
            }
        }
    }

    deps
}

/// Parse requirements.txt: one dep per non-blank, non-comment line.
fn parse_requirements(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('-'))
        .map(|l| {
            // Strip version specifiers: ==, >=, <=, ~=, !=, [extras]
            let name = l
                .split(&['=', '>', '<', '!', '~', '[', ';'][..])
                .next()
                .unwrap_or(l)
                .trim();
            name.to_string()
        })
        .filter(|n| !n.is_empty())
        .collect()
}

/// Parse Pipfile: extract package names from [packages] and [dev-packages].
fn parse_pipfile(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_section = trimmed == "[packages]" || trimmed == "[dev-packages]";
            continue;
        }
        if in_section {
            if let Some(name) = trimmed.split('=').next() {
                let name = name.trim();
                if !name.is_empty() && !name.starts_with('#') {
                    deps.push(name.to_string());
                }
            }
        }
    }

    deps
}

/// Parse go.mod: extract module paths from require block.
fn parse_go_mod(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_require = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("require (") || trimmed == "require (" {
            in_require = true;
            continue;
        }

        if in_require {
            if trimmed == ")" {
                in_require = false;
                continue;
            }
            if let Some(module) = trimmed.split_whitespace().next() {
                if !module.starts_with("//") {
                    deps.push(module.to_string());
                }
            }
        }

        // Single-line require
        if trimmed.starts_with("require ") && !trimmed.contains('(') {
            if let Some(module) = trimmed.strip_prefix("require ") {
                if let Some(m) = module.split_whitespace().next() {
                    deps.push(m.to_string());
                }
            }
        }
    }

    deps
}

/// Parse Gemfile: extract gem names from `gem "name"` lines.
fn parse_gemfile(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.starts_with("gem "))
        .filter_map(|l| {
            let rest = l.strip_prefix("gem ")?;
            let name = rest
                .trim_start_matches(&['"', '\''][..])
                .split(&['"', '\'', ','][..])
                .next()?;
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}

/// Parse pom.xml: extract artifactId from dependency blocks (basic regex-like).
fn parse_pom_xml(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_dependency = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.contains("<dependency>") {
            in_dependency = true;
        }
        if in_dependency && trimmed.contains("<artifactId>") {
            if let Some(start) = trimmed.find("<artifactId>") {
                let after = &trimmed[start + 12..];
                if let Some(end) = after.find("</artifactId>") {
                    deps.push(after[..end].to_string());
                }
            }
        }
        if trimmed.contains("</dependency>") {
            in_dependency = false;
        }
    }

    deps
}

/// Parse build.gradle: extract deps from implementation/api/compile lines.
fn parse_gradle(content: &str) -> Vec<String> {
    let prefixes = [
        "implementation",
        "api",
        "compile",
        "testImplementation",
        "runtimeOnly",
    ];

    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| prefixes.iter().any(|p| l.starts_with(p)))
        .filter_map(|l| {
            // Extract from quotes: implementation "group:artifact:version"
            let start = l.find(&['"', '\''][..])?;
            let rest = &l[start + 1..];
            let end = rest.find(&['"', '\''][..])?;
            let dep = &rest[..end];
            // Take group:artifact (skip version)
            let parts: Vec<&str> = dep.split(':').collect();
            if parts.len() >= 2 {
                Some(format!("{}:{}", parts[0], parts[1]))
            } else {
                Some(dep.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn parses_cargo_toml() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("Cargo.toml");
        fs::write(
            &path,
            r#"
[package]
name = "myapp"

[dependencies]
serde = "1"
clap = { version = "4" }

[dev-dependencies]
tempfile = "3"
"#,
        )
        .unwrap();

        let info = parse_manifest(&path).unwrap();
        assert_eq!(info.manifest_type, ManifestType::Cargo);
        assert!(info.direct_deps.contains(&"serde".to_string()));
        assert!(info.direct_deps.contains(&"clap".to_string()));
        assert!(info.direct_deps.contains(&"tempfile".to_string()));
    }

    #[test]
    fn parses_package_json() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("package.json");
        fs::write(
            &path,
            r#"{
  "name": "myapp",
  "dependencies": {
    "react": "^18.0.0",
    "next": "^14.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}"#,
        )
        .unwrap();

        let info = parse_manifest(&path).unwrap();
        assert_eq!(info.manifest_type, ManifestType::Npm);
        assert!(info.direct_deps.contains(&"react".to_string()));
        assert!(info.direct_deps.contains(&"next".to_string()));
        assert!(info.direct_deps.contains(&"typescript".to_string()));
    }

    #[test]
    fn parses_requirements_txt() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("requirements.txt");
        fs::write(&path, "flask==2.0\nrequests>=2.28\n# comment\n\npytest\n").unwrap();

        let info = parse_manifest(&path).unwrap();
        assert_eq!(info.manifest_type, ManifestType::Python);
        assert_eq!(info.direct_deps.len(), 3);
        assert!(info.direct_deps.contains(&"flask".to_string()));
        assert!(info.direct_deps.contains(&"requests".to_string()));
        assert!(info.direct_deps.contains(&"pytest".to_string()));
    }

    #[test]
    fn parses_go_mod() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("go.mod");
        fs::write(
            &path,
            "module example.com/myapp\n\ngo 1.21\n\nrequire (\n\tgithub.com/gin-gonic/gin v1.9\n\tgithub.com/stretchr/testify v1.8\n)\n",
        )
        .unwrap();

        let info = parse_manifest(&path).unwrap();
        assert_eq!(info.manifest_type, ManifestType::Go);
        assert_eq!(info.direct_deps.len(), 2);
    }

    #[test]
    fn returns_none_for_unknown_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("random.txt");
        fs::write(&path, "hello").unwrap();
        assert!(parse_manifest(&path).is_none());
    }

    #[test]
    fn handles_empty_manifest() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("Cargo.toml");
        fs::write(&path, "[package]\nname = \"empty\"\n").unwrap();

        let info = parse_manifest(&path).unwrap();
        assert!(info.direct_deps.is_empty());
    }

    #[test]
    fn finds_manifests_in_directory() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();
        fs::create_dir_all(dir.path().join("frontend")).unwrap();
        fs::write(dir.path().join("frontend/package.json"), r#"{"name":"fe"}"#).unwrap();
        // Should be excluded
        fs::create_dir_all(dir.path().join("node_modules/pkg")).unwrap();
        fs::write(
            dir.path().join("node_modules/pkg/package.json"),
            r#"{"name":"pkg"}"#,
        )
        .unwrap();

        let manifests = find_manifests(dir.path());
        assert_eq!(manifests.len(), 2);
        assert!(manifests.iter().any(|p| p.ends_with("Cargo.toml")));
        assert!(manifests.iter().any(|p| p.ends_with("package.json")));
    }

    #[test]
    fn summarizes_dependencies() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("Cargo.toml"),
            "[dependencies]\nserde = \"1\"\nclap = \"4\"\n",
        )
        .unwrap();
        fs::create_dir_all(dir.path().join("frontend")).unwrap();
        fs::write(
            dir.path().join("frontend/package.json"),
            r#"{"dependencies":{"react":"18","next":"14"}}"#,
        )
        .unwrap();

        let summary = summarize_dependencies(dir.path());
        assert_eq!(summary.manifests.len(), 2);
        assert_eq!(summary.total_direct, 4); // 2 cargo + 2 npm
    }

    #[test]
    fn counts_cargo_lock_packages() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("Cargo.lock"),
            "[[package]]\nname = \"a\"\n\n[[package]]\nname = \"b\"\n\n[[package]]\nname = \"c\"\n",
        )
        .unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[dependencies]\na = \"1\"\n").unwrap();

        let summary = summarize_dependencies(dir.path());
        assert_eq!(summary.total_transitive, Some(3));
    }

    #[test]
    fn transitive_none_without_lock_file() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[dependencies]\na = \"1\"\n").unwrap();

        let summary = summarize_dependencies(dir.path());
        assert!(summary.total_transitive.is_none());
    }
}
