//! Tree-sitter integration for AST parsing and complexity analysis.

use tree_sitter::{Node, Parser, Tree};

use crate::scanner::language::Language;

/// Cyclomatic complexity result for a file.
#[derive(Debug, Clone, Default)]
pub(crate) struct FileComplexity {
    /// Total cyclomatic complexity across all functions.
    pub(crate) total: usize,
    /// Number of functions analyzed.
    pub(crate) function_count: usize,
    /// Average complexity per function (0.0 if no functions).
    pub(crate) average: f64,
}

/// Parse source code into a tree-sitter syntax tree.
///
/// Returns `None` if the language has no tree-sitter grammar available.
pub(crate) fn parse(content: &str, language: Language) -> Option<Tree> {
    let ts_language = grammar_for(language)?;
    let mut parser = Parser::new();
    parser.set_language(&ts_language).ok()?;
    parser.parse(content, None)
}

/// Calculate cyclomatic complexity for a file.
///
/// Cyclomatic complexity = 1 + number of decision points per function.
/// Decision points: if, else if, while, for, case, &&, ||, catch/except, ternary.
pub(crate) fn cyclomatic_complexity(content: &str, language: Language) -> Option<FileComplexity> {
    let tree = parse(content, language)?;
    let root = tree.root_node();
    let content_bytes = content.as_bytes();

    let mut total = 0;
    let mut function_count = 0;

    collect_function_complexities(root, content_bytes, &mut total, &mut function_count);

    // If no functions found, treat the whole file as one unit
    if function_count == 0 {
        let complexity = count_decision_points(root, content_bytes);
        return Some(FileComplexity {
            total: 1 + complexity,
            function_count: 0,
            average: 0.0,
        });
    }

    let average = if function_count > 0 {
        total as f64 / function_count as f64
    } else {
        0.0
    };

    Some(FileComplexity {
        total,
        function_count,
        average,
    })
}

/// Recursively find function nodes and compute their complexity.
fn collect_function_complexities(node: Node, source: &[u8], total: &mut usize, count: &mut usize) {
    if is_function_node(node.kind()) {
        let complexity = 1 + count_decision_points(node, source);
        *total += complexity;
        *count += 1;
        return; // Don't recurse into nested functions separately
    }

    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            collect_function_complexities(child, source, total, count);
        }
    }
}

/// Check if a node kind represents a function definition.
fn is_function_node(kind: &str) -> bool {
    matches!(
        kind,
        "function_definition"
            | "function_item"
            | "function_declaration"
            | "method_definition"
            | "method_declaration"
            | "arrow_function"
            | "lambda"
            | "lambda_expression"
            | "closure_expression"
    )
}

/// Count decision points in a subtree.
fn count_decision_points(node: Node, source: &[u8]) -> usize {
    let mut count = 0;

    if is_decision_point(node.kind(), node, source) {
        count += 1;
    }

    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            // Don't recurse into nested function definitions
            if !is_function_node(child.kind()) {
                count += count_decision_points(child, source);
            }
        }
    }

    count
}

/// Check if a node kind is a decision point for cyclomatic complexity.
fn is_decision_point(kind: &str, node: Node, source: &[u8]) -> bool {
    match kind {
        // Branching
        "if_expression" | "if_statement" | "if_let_expression" | "elif_clause"
        | "else_if_clause" | "guard_statement" => true,
        // Loops
        "while_expression"
        | "while_statement"
        | "while_let_expression"
        | "for_expression"
        | "for_statement"
        | "for_in_statement"
        | "loop_expression" => true,
        // Pattern matching cases
        "match_arm" | "switch_case" | "case_clause" | "case_statement" | "when_clause" => true,
        // Exception handling
        "catch_clause" | "except_clause" | "rescue" => true,
        // Ternary
        "ternary_expression" | "conditional_expression" => true,
        // Logical operators (short-circuit)
        "binary_expression" => {
            let text = node
                .utf8_text(source)
                .map(|t| t.to_string())
                .unwrap_or_default();
            // Check if the operator is && or ||
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "&&" || child.kind() == "||" {
                        return true;
                    }
                    let child_text = child.utf8_text(source).unwrap_or("");
                    if child_text == "&&"
                        || child_text == "||"
                        || child_text == "and"
                        || child_text == "or"
                    {
                        return true;
                    }
                }
            }
            // Also check for boolean operators in languages that use words
            text.contains(" and ") || text.contains(" or ")
        }
        "boolean_operator" => true,
        _ => false,
    }
}

/// Map a Language to its tree-sitter grammar, if available.
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

    // --- Cyclomatic complexity tests ---

    #[test]
    fn simple_function_has_complexity_1() {
        let code = "fn hello() { println!(\"hi\"); }";
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.total, 1, "simple function = complexity 1");
        assert_eq!(result.function_count, 1);
    }

    #[test]
    fn function_with_if_has_complexity_2() {
        let code = "fn check(x: i32) { if x > 0 { return; } }";
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.total, 2, "one if = complexity 2");
    }

    #[test]
    fn function_with_if_else_if_has_complexity_3() {
        let code = r#"
fn classify(x: i32) -> &'static str {
    if x > 0 {
        "positive"
    } else if x < 0 {
        "negative"
    } else {
        "zero"
    }
}
"#;
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert!(
            result.total >= 3,
            "if + else if = at least 3, got {}",
            result.total
        );
    }

    #[test]
    fn function_with_loop_has_complexity_2() {
        let code = "fn loopy() { for i in 0..10 { println!(\"{}\", i); } }";
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.total, 2, "one loop = complexity 2");
    }

    #[test]
    fn python_function_with_if() {
        let code = "def check(x):\n    if x > 0:\n        return True\n    return False\n";
        let result = cyclomatic_complexity(code, Language::Python).unwrap();
        assert_eq!(result.total, 2);
    }

    #[test]
    fn multiple_functions_sum_correctly() {
        let code = r#"
fn a() { if true { } }
fn b() { if true { } if true { } }
"#;
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.function_count, 2);
        // a = 2, b = 3 => total = 5
        assert_eq!(result.total, 5);
    }

    #[test]
    fn average_complexity_calculated() {
        let code = r#"
fn simple() { }
fn complex() { if true { } if true { } }
"#;
        let result = cyclomatic_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.function_count, 2);
        // simple=1, complex=3 => avg=2.0
        assert!((result.average - 2.0).abs() < 0.01);
    }

    #[test]
    fn unsupported_language_returns_none() {
        let result = cyclomatic_complexity("SELECT 1;", Language::SQL);
        assert!(result.is_none());
    }
}
