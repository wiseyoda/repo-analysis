//! Language detection from file extensions.

use std::path::Path;

/// A programming language identified by file extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Language {
    /// Rust (.rs)
    Rust,
    /// TypeScript (.ts, .tsx)
    TypeScript,
    /// JavaScript (.js, .jsx, .mjs, .cjs)
    JavaScript,
    /// Python (.py, .pyi)
    Python,
    /// Go (.go)
    Go,
    /// Swift (.swift)
    Swift,
    /// Java (.java)
    Java,
    /// C (.c, .h)
    C,
    /// C++ (.cpp, .cc, .cxx, .hpp, .hxx)
    Cpp,
    /// C# (.cs)
    CSharp,
    /// Ruby (.rb)
    Ruby,
    /// PHP (.php)
    PHP,
    /// Kotlin (.kt, .kts)
    Kotlin,
    /// Scala (.scala, .sc)
    Scala,
    /// Haskell (.hs, .lhs)
    Haskell,
    /// Elixir (.ex, .exs)
    Elixir,
    /// Erlang (.erl, .hrl)
    Erlang,
    /// Clojure (.clj, .cljs, .cljc, .edn)
    Clojure,
    /// Lua (.lua)
    Lua,
    /// Perl (.pl, .pm)
    Perl,
    /// R (.r, .R)
    R,
    /// Julia (.jl)
    Julia,
    /// Dart (.dart)
    Dart,
    /// Objective-C (.m, .mm)
    ObjectiveC,
    /// Shell (.sh)
    Shell,
    /// Bash (.bash)
    Bash,
    /// Zsh (.zsh)
    Zsh,
    /// Fish (.fish)
    Fish,
    /// PowerShell (.ps1, .psm1)
    PowerShell,
    /// SQL (.sql)
    SQL,
    /// HTML (.html, .htm)
    HTML,
    /// CSS (.css)
    CSS,
    /// SCSS (.scss)
    SCSS,
    /// Less (.less)
    Less,
    /// Sass (.sass)
    Sass,
    /// XML (.xml, .xsl, .xslt, .svg)
    XML,
    /// JSON (.json)
    JSON,
    /// YAML (.yaml, .yml)
    YAML,
    /// TOML (.toml)
    TOML,
    /// Markdown (.md, .markdown)
    Markdown,
    /// Zig (.zig)
    Zig,
    /// Nim (.nim)
    Nim,
    /// OCaml (.ml, .mli)
    OCaml,
    /// F# (.fs, .fsx, .fsi)
    FSharp,
    /// Groovy (.groovy, .gvy)
    Groovy,
    /// Terraform (.tf, .tfvars)
    Terraform,
    /// Dockerfile
    Dockerfile,
    /// Makefile
    Makefile,
    /// CMake (.cmake, CMakeLists.txt)
    CMake,
    /// Protocol Buffers (.proto)
    Protobuf,
    /// GraphQL (.graphql, .gql)
    GraphQL,
    /// Vue (.vue)
    Vue,
    /// Svelte (.svelte)
    Svelte,
}

impl Language {
    /// Detect language from a file path based on extension or filename.
    pub(crate) fn detect(path: &Path) -> Option<Self> {
        // Try special filenames first
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(lang) = Self::from_filename(name) {
                return Some(lang);
            }
        }

        // Then try extension
        let ext = path.extension()?.to_str()?.to_lowercase();
        Self::from_extension(&ext)
    }

    /// Human-readable display name for this language.
    #[allow(dead_code)] // used by report/dashboard (upcoming task)
    pub(crate) fn display_name(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Python => "Python",
            Self::Go => "Go",
            Self::Swift => "Swift",
            Self::Java => "Java",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::Ruby => "Ruby",
            Self::PHP => "PHP",
            Self::Kotlin => "Kotlin",
            Self::Scala => "Scala",
            Self::Haskell => "Haskell",
            Self::Elixir => "Elixir",
            Self::Erlang => "Erlang",
            Self::Clojure => "Clojure",
            Self::Lua => "Lua",
            Self::Perl => "Perl",
            Self::R => "R",
            Self::Julia => "Julia",
            Self::Dart => "Dart",
            Self::ObjectiveC => "Objective-C",
            Self::Shell => "Shell",
            Self::Bash => "Bash",
            Self::Zsh => "Zsh",
            Self::Fish => "Fish",
            Self::PowerShell => "PowerShell",
            Self::SQL => "SQL",
            Self::HTML => "HTML",
            Self::CSS => "CSS",
            Self::SCSS => "SCSS",
            Self::Less => "Less",
            Self::Sass => "Sass",
            Self::XML => "XML",
            Self::JSON => "JSON",
            Self::YAML => "YAML",
            Self::TOML => "TOML",
            Self::Markdown => "Markdown",
            Self::Zig => "Zig",
            Self::Nim => "Nim",
            Self::OCaml => "OCaml",
            Self::FSharp => "F#",
            Self::Groovy => "Groovy",
            Self::Terraform => "Terraform",
            Self::Dockerfile => "Dockerfile",
            Self::Makefile => "Makefile",
            Self::CMake => "CMake",
            Self::Protobuf => "Protocol Buffers",
            Self::GraphQL => "GraphQL",
            Self::Vue => "Vue",
            Self::Svelte => "Svelte",
        }
    }

    /// Match special filenames that don't use extensions.
    fn from_filename(name: &str) -> Option<Self> {
        match name {
            "Makefile" | "GNUmakefile" | "makefile" => Some(Self::Makefile),
            "Dockerfile" => Some(Self::Dockerfile),
            "CMakeLists.txt" => Some(Self::CMake),
            _ => None,
        }
    }

    /// Match a lowercase file extension to a language.
    fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Self::Rust),
            "ts" | "tsx" => Some(Self::TypeScript),
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "py" | "pyi" => Some(Self::Python),
            "go" => Some(Self::Go),
            "swift" => Some(Self::Swift),
            "java" => Some(Self::Java),
            "c" | "h" => Some(Self::C),
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Self::Cpp),
            "cs" => Some(Self::CSharp),
            "rb" => Some(Self::Ruby),
            "php" => Some(Self::PHP),
            "kt" | "kts" => Some(Self::Kotlin),
            "scala" | "sc" => Some(Self::Scala),
            "hs" | "lhs" => Some(Self::Haskell),
            "ex" | "exs" => Some(Self::Elixir),
            "erl" | "hrl" => Some(Self::Erlang),
            "clj" | "cljs" | "cljc" | "edn" => Some(Self::Clojure),
            "lua" => Some(Self::Lua),
            "pl" | "pm" => Some(Self::Perl),
            "r" => Some(Self::R),
            "jl" => Some(Self::Julia),
            "dart" => Some(Self::Dart),
            "m" | "mm" => Some(Self::ObjectiveC),
            "sh" => Some(Self::Shell),
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "fish" => Some(Self::Fish),
            "ps1" | "psm1" => Some(Self::PowerShell),
            "sql" => Some(Self::SQL),
            "html" | "htm" => Some(Self::HTML),
            "css" => Some(Self::CSS),
            "scss" => Some(Self::SCSS),
            "less" => Some(Self::Less),
            "sass" => Some(Self::Sass),
            "xml" | "xsl" | "xslt" | "svg" => Some(Self::XML),
            "json" => Some(Self::JSON),
            "yaml" | "yml" => Some(Self::YAML),
            "toml" => Some(Self::TOML),
            "md" | "markdown" => Some(Self::Markdown),
            "zig" => Some(Self::Zig),
            "nim" => Some(Self::Nim),
            "ml" | "mli" => Some(Self::OCaml),
            "fs" | "fsx" | "fsi" => Some(Self::FSharp),
            "groovy" | "gvy" => Some(Self::Groovy),
            "tf" | "tfvars" => Some(Self::Terraform),
            "proto" => Some(Self::Protobuf),
            "graphql" | "gql" => Some(Self::GraphQL),
            "vue" => Some(Self::Vue),
            "svelte" => Some(Self::Svelte),
            "cmake" => Some(Self::CMake),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn detects_rust() {
        assert_eq!(Language::detect(Path::new("main.rs")), Some(Language::Rust));
    }

    #[test]
    fn detects_typescript_ts_and_tsx() {
        assert_eq!(
            Language::detect(Path::new("app.ts")),
            Some(Language::TypeScript)
        );
        assert_eq!(
            Language::detect(Path::new("App.tsx")),
            Some(Language::TypeScript)
        );
    }

    #[test]
    fn detects_python() {
        assert_eq!(
            Language::detect(Path::new("script.py")),
            Some(Language::Python)
        );
    }

    #[test]
    fn detects_c_and_cpp() {
        assert_eq!(Language::detect(Path::new("main.c")), Some(Language::C));
        assert_eq!(Language::detect(Path::new("main.cpp")), Some(Language::Cpp));
        assert_eq!(Language::detect(Path::new("main.cc")), Some(Language::Cpp));
        assert_eq!(Language::detect(Path::new("main.h")), Some(Language::C));
        assert_eq!(Language::detect(Path::new("main.hpp")), Some(Language::Cpp));
    }

    #[test]
    fn detects_special_filenames() {
        assert_eq!(
            Language::detect(Path::new("Makefile")),
            Some(Language::Makefile)
        );
        assert_eq!(
            Language::detect(Path::new("Dockerfile")),
            Some(Language::Dockerfile)
        );
        assert_eq!(
            Language::detect(Path::new("CMakeLists.txt")),
            Some(Language::CMake)
        );
    }

    #[test]
    fn case_insensitive_extension() {
        assert_eq!(Language::detect(Path::new("Main.RS")), Some(Language::Rust));
        assert_eq!(
            Language::detect(Path::new("App.PY")),
            Some(Language::Python)
        );
    }

    #[test]
    fn returns_none_for_unknown_extension() {
        assert_eq!(Language::detect(Path::new("file.xyz123")), None);
    }

    #[test]
    fn returns_none_for_no_extension() {
        assert_eq!(Language::detect(Path::new("README")), None);
    }

    #[test]
    fn display_name_works() {
        assert_eq!(Language::Rust.display_name(), "Rust");
        assert_eq!(Language::Cpp.display_name(), "C++");
        assert_eq!(Language::CSharp.display_name(), "C#");
        assert_eq!(Language::ObjectiveC.display_name(), "Objective-C");
        assert_eq!(Language::FSharp.display_name(), "F#");
    }

    #[test]
    fn covers_at_least_50_languages() {
        // Count unique variants by checking a known set of extensions
        let extensions = [
            "rs", "ts", "tsx", "js", "jsx", "py", "go", "swift", "java", "c", "cpp", "cs", "rb",
            "php", "kt", "scala", "hs", "ex", "erl", "clj", "lua", "pl", "r", "jl", "dart", "m",
            "sh", "bash", "zsh", "fish", "ps1", "sql", "html", "css", "scss", "less", "sass",
            "xml", "json", "yaml", "toml", "md", "zig", "nim", "ml", "fs", "groovy", "tf", "proto",
            "graphql", "vue", "svelte",
        ];
        let detected: std::collections::HashSet<_> = extensions
            .iter()
            .filter_map(|ext| Language::detect(Path::new(&format!("file.{ext}"))))
            .collect();
        assert!(
            detected.len() >= 50,
            "expected 50+ languages, got {}",
            detected.len()
        );
    }
}
