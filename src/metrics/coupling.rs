//! Internal coupling analysis: import/require/use statement parsing.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::scanner::language::Language;
use crate::scanner::ScannedFile;

/// Internal dependency graph: which files import which.
#[derive(Debug, Clone, Default)]
pub(crate) struct CouplingGraph {
    /// Map of file path → set of imported module/file paths.
    pub(crate) imports: BTreeMap<PathBuf, BTreeSet<String>>,
}

/// Fan-in / fan-out metrics for a module.
#[derive(Debug, Clone, Default)]
pub(crate) struct ModuleCoupling {
    /// Number of modules this module depends on (outgoing).
    pub(crate) fan_out: usize,
    /// Number of modules that depend on this module (incoming).
    pub(crate) fan_in: usize,
}

/// Calculate fan-in and fan-out for each module in the coupling graph.
pub(crate) fn calculate_fan_metrics(graph: &CouplingGraph) -> BTreeMap<PathBuf, ModuleCoupling> {
    let mut metrics: BTreeMap<PathBuf, ModuleCoupling> = BTreeMap::new();

    // Fan-out: how many unique modules each file imports
    for (file, imports) in &graph.imports {
        metrics.entry(file.clone()).or_default().fan_out = imports.len();
    }

    // Fan-in: how many files import each module
    // Match import strings to known files by module name
    let known_modules: BTreeSet<String> = graph
        .imports
        .keys()
        .filter_map(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    for imports in graph.imports.values() {
        for import in imports {
            // Extract the leaf module name from the import path
            let module_name = import.rsplit(&['/', '.', ':'][..]).next().unwrap_or(import);
            // Find files whose stem matches this module name
            for path in graph.imports.keys() {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem == module_name && known_modules.contains(stem) {
                        metrics.entry(path.clone()).or_default().fan_in += 1;
                    }
                }
            }
        }
    }

    metrics
}

/// Extract import statements from a file's content based on its language.
pub(crate) fn extract_imports(content: &str, language: Language) -> Vec<String> {
    match language {
        Language::Rust => extract_rust_imports(content),
        Language::Python => extract_python_imports(content),
        Language::JavaScript | Language::TypeScript => extract_js_imports(content),
        Language::Go => extract_go_imports(content),
        Language::Java | Language::Kotlin | Language::Scala => extract_java_imports(content),
        Language::Ruby => extract_ruby_imports(content),
        Language::CSharp => extract_csharp_imports(content),
        Language::Swift => extract_swift_imports(content),
        _ => Vec::new(),
    }
}

/// Build a coupling graph from scanned files.
pub(crate) fn build_coupling_graph(files: &[ScannedFile], root: &Path) -> CouplingGraph {
    let mut imports = BTreeMap::new();

    for file in files {
        let Some(language) = file.language else {
            continue;
        };

        let Ok(content) = std::fs::read_to_string(&file.path) else {
            continue;
        };

        let file_imports = extract_imports(&content, language);
        if !file_imports.is_empty() {
            let relative = file.path.strip_prefix(root).unwrap_or(&file.path);
            imports.insert(relative.to_path_buf(), file_imports.into_iter().collect());
        }
    }

    CouplingGraph { imports }
}

/// Extract Rust `use crate::` and `mod` statements.
fn extract_rust_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            if l.starts_with("use crate::") {
                let module = l
                    .strip_prefix("use crate::")?
                    .split(&[':', ';', ' ', '{'][..])
                    .next()?;
                Some(module.to_string())
            } else if l.starts_with("mod ") && !l.contains("test") {
                let module = l.strip_prefix("mod ")?.trim_end_matches(';').trim();
                Some(module.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract Python `import` and `from ... import` statements.
fn extract_python_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            if l.starts_with("from ") {
                let module = l.strip_prefix("from ")?.split_whitespace().next()?;
                if !module.starts_with('.') {
                    return Some(module.to_string());
                }
                Some(module.to_string())
            } else if l.starts_with("import ") {
                let module = l
                    .strip_prefix("import ")?
                    .split(&[',', ' ', '.'][..])
                    .next()?;
                Some(module.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract JS/TS `import` and `require` statements.
fn extract_js_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            if l.contains("import ") && l.contains("from ") {
                let from_part = l.rsplit("from ").next()?;
                let module = from_part.trim().trim_matches(&['"', '\'', ';', ' '][..]);
                Some(module.to_string())
            } else if l.contains("require(") {
                let start = l.find("require(")? + 8;
                let rest = &l[start..];
                let module = rest
                    .trim_start_matches(&['"', '\''][..])
                    .split(&['"', '\'', ')'][..])
                    .next()?;
                Some(module.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract Go `import` statements.
fn extract_go_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    let mut in_import_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "import (" {
            in_import_block = true;
            continue;
        }
        if in_import_block {
            if trimmed == ")" {
                in_import_block = false;
                continue;
            }
            let module = trimmed.trim_matches(&['"', ' ', '\t'][..]);
            if !module.is_empty() {
                imports.push(module.to_string());
            }
        }
        if trimmed.starts_with("import \"") {
            if let Some(rest) = trimmed.strip_prefix("import ") {
                let module = rest.trim_matches('"');
                imports.push(module.to_string());
            }
        }
    }

    imports
}

/// Extract Java/Kotlin/Scala `import` statements.
fn extract_java_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.starts_with("import "))
        .filter_map(|l| {
            let module = l
                .strip_prefix("import ")?
                .trim_end_matches(';')
                .split_whitespace()
                .next()?;
            Some(module.to_string())
        })
        .collect()
}

/// Extract Ruby `require` statements.
fn extract_ruby_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            if l.starts_with("require ") || l.starts_with("require_relative ") {
                let module = l.split(&['"', '\''][..]).nth(1)?;
                Some(module.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract C# `using` statements.
fn extract_csharp_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.starts_with("using ") && l.ends_with(';'))
        .filter_map(|l| {
            let module = l.strip_prefix("using ")?.trim_end_matches(';').trim();
            if module.contains('=') {
                None // Skip aliases
            } else {
                Some(module.to_string())
            }
        })
        .collect()
}

/// Extract Swift `import` statements.
fn extract_swift_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.starts_with("import "))
        .filter_map(|l| {
            let module = l.strip_prefix("import ")?.split_whitespace().next()?;
            Some(module.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_rust_use_crate() {
        let code = "use crate::config::Config;\nuse crate::scanner;\n";
        let imports = extract_imports(code, Language::Rust);
        assert!(imports.contains(&"config".to_string()));
        assert!(imports.contains(&"scanner".to_string()));
    }

    #[test]
    fn extracts_python_imports() {
        let code = "import os\nfrom pathlib import Path\nimport json\n";
        let imports = extract_imports(code, Language::Python);
        assert_eq!(imports.len(), 3);
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"pathlib".to_string()));
    }

    #[test]
    fn extracts_js_imports() {
        let code = "import React from 'react';\nconst fs = require('fs');\n";
        let imports = extract_imports(code, Language::JavaScript);
        assert!(imports.contains(&"react".to_string()));
        assert!(imports.contains(&"fs".to_string()));
    }

    #[test]
    fn extracts_go_imports() {
        let code = "import (\n\t\"fmt\"\n\t\"os\"\n)\n";
        let imports = extract_imports(code, Language::Go);
        assert!(imports.contains(&"fmt".to_string()));
        assert!(imports.contains(&"os".to_string()));
    }

    #[test]
    fn extracts_java_imports() {
        let code = "import java.util.List;\nimport java.io.File;\n";
        let imports = extract_imports(code, Language::Java);
        assert_eq!(imports.len(), 2);
    }

    #[test]
    fn returns_empty_for_unsupported() {
        let imports = extract_imports("SELECT 1;", Language::SQL);
        assert!(imports.is_empty());
    }

    #[test]
    fn builds_coupling_graph() {
        use tempfile::TempDir;

        let dir = TempDir::new().unwrap();
        let rs_path = dir.path().join("main.rs");
        std::fs::write(&rs_path, "use crate::config::Config;\nfn main() {}\n").unwrap();

        let files = vec![ScannedFile {
            path: rs_path,
            language: Some(Language::Rust),
            is_minified: false,
            is_generated: false,
        }];

        let graph = build_coupling_graph(&files, dir.path());
        assert!(!graph.imports.is_empty());
    }

    #[test]
    fn calculates_fan_out() {
        let mut imports = BTreeMap::new();
        let mut set = BTreeSet::new();
        set.insert("config".to_string());
        set.insert("scanner".to_string());
        imports.insert(PathBuf::from("main.rs"), set);

        let graph = CouplingGraph { imports };
        let metrics = calculate_fan_metrics(&graph);

        assert_eq!(metrics[&PathBuf::from("main.rs")].fan_out, 2);
    }

    #[test]
    fn empty_graph_has_no_metrics() {
        let graph = CouplingGraph::default();
        let metrics = calculate_fan_metrics(&graph);
        assert!(metrics.is_empty());
    }
}
