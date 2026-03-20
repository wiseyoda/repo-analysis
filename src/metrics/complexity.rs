//! Tree-sitter integration for AST parsing and complexity analysis.

use tree_sitter::{Parser, Tree};

use crate::scanner::language::Language;

/// Parse source code into a tree-sitter syntax tree.
///
/// Returns `None` if the language has no tree-sitter grammar available.
#[allow(dead_code)] // called by cyclomatic/cognitive complexity (next tasks)
pub(crate) fn parse(content: &str, language: Language) -> Option<Tree> {
    let ts_language = grammar_for(language)?;
    let mut parser = Parser::new();
    parser.set_language(&ts_language).ok()?;
    parser.parse(content, None)
}

/// Map a Language to its tree-sitter grammar, if available.
#[allow(dead_code)] // called by parse()
fn grammar_for(language: Language) -> Option<tree_sitter::Language> {
    use Language::*;
    match language {
        Rust => Some(tree_sitter_rust::LANGUAGE.into()),
        Python => Some(tree_sitter_python::LANGUAGE.into()),
        JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
        TypeScript => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        Go => Some(tree_sitter_go::LANGUAGE.into()),
        Java => Some(tree_sitter_java::LANGUAGE.into()),
        C => Some(tree_sitter_c::LANGUAGE.into()),
        Cpp => Some(tree_sitter_cpp::LANGUAGE.into()),
        Swift => Some(tree_sitter_swift::LANGUAGE.into()),
        Ruby => Some(tree_sitter_ruby::LANGUAGE.into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::language::Language;

    use super::*;

    #[test]
    fn parses_rust_source() {
        let code = "fn main() { let x = 1; }";
        let tree = parse(code, Language::Rust);
        assert!(tree.is_some(), "should parse Rust code");
        let tree = tree.unwrap();
        assert!(
            tree.root_node().child_count() > 0,
            "tree should have children"
        );
    }

    #[test]
    fn parses_python_source() {
        let code = "def hello():\n    print('hello')\n";
        let tree = parse(code, Language::Python);
        assert!(tree.is_some(), "should parse Python code");
    }

    #[test]
    fn parses_javascript_source() {
        let code = "function foo() { return 1; }";
        let tree = parse(code, Language::JavaScript);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_typescript_source() {
        let code = "const x: number = 1;";
        let tree = parse(code, Language::TypeScript);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_go_source() {
        let code = "package main\nfunc main() {}";
        let tree = parse(code, Language::Go);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_java_source() {
        let code = "class Main { public static void main(String[] args) {} }";
        let tree = parse(code, Language::Java);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_c_source() {
        let code = "int main() { return 0; }";
        let tree = parse(code, Language::C);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_cpp_source() {
        let code = "int main() { return 0; }";
        let tree = parse(code, Language::Cpp);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_swift_source() {
        let code = "func greet() { print(\"hello\") }";
        let tree = parse(code, Language::Swift);
        assert!(tree.is_some());
    }

    #[test]
    fn parses_ruby_source() {
        let code = "def hello\n  puts 'hello'\nend";
        let tree = parse(code, Language::Ruby);
        assert!(tree.is_some());
    }

    #[test]
    fn returns_none_for_unsupported_language() {
        let code = "SELECT 1;";
        let tree = parse(code, Language::SQL);
        assert!(tree.is_none(), "SQL has no tree-sitter grammar");
    }

    #[test]
    fn handles_empty_content() {
        let tree = parse("", Language::Rust);
        assert!(tree.is_some(), "tree-sitter should handle empty input");
    }
}
