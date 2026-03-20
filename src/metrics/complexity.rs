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

/// Cognitive complexity result for a file.
///
/// SonarQube-style: each control flow break adds 1, plus the current nesting depth.
#[derive(Debug, Clone, Default)]
pub(crate) struct CognitiveComplexity {
    /// Total cognitive complexity across all functions.
    pub(crate) total: usize,
    /// Number of functions analyzed.
    pub(crate) function_count: usize,
    /// Average cognitive complexity per function.
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

/// Calculate cognitive complexity for a file (SonarQube-style).
///
/// Increments: +1 for each flow break, +nesting_depth for nested flow breaks.
pub(crate) fn cognitive_complexity(
    content: &str,
    language: Language,
) -> Option<CognitiveComplexity> {
    let tree = parse(content, language)?;
    let root = tree.root_node();

    let mut total = 0;
    let mut function_count = 0;

    collect_cognitive_complexities(root, &mut total, &mut function_count);

    if function_count == 0 {
        let score = cognitive_score(root, 0);
        return Some(CognitiveComplexity {
            total: score,
            function_count: 0,
            average: 0.0,
        });
    }

    let average = if function_count > 0 {
        total as f64 / function_count as f64
    } else {
        0.0
    };

    Some(CognitiveComplexity {
        total,
        function_count,
        average,
    })
}

/// Default threshold for flagging large functions (lines).
pub(crate) const DEFAULT_FUNCTION_SIZE_THRESHOLD: usize = 50;

/// Default threshold for flagging large files (lines).
pub(crate) const DEFAULT_FILE_SIZE_THRESHOLD: usize = 500;

/// A flagged item that exceeds a threshold.
#[derive(Debug, Clone)]
pub(crate) struct ThresholdViolation {
    /// Name of the file or function.
    pub(crate) name: String,
    /// The measured value (lines or complexity).
    pub(crate) value: usize,
    /// The threshold that was exceeded.
    pub(crate) threshold: usize,
    /// What kind of violation this is.
    pub(crate) kind: ViolationKind,
}

/// The type of threshold violation.
#[derive(Debug, Clone)]
pub(crate) enum ViolationKind {
    /// Function exceeds line count threshold.
    LargeFunction,
    /// File exceeds line count threshold.
    LargeFile,
}

/// Check functions for size threshold violations.
pub(crate) fn flag_large_functions(
    functions: &[FunctionInfo],
    threshold: usize,
) -> Vec<ThresholdViolation> {
    functions
        .iter()
        .filter(|f| f.line_count > threshold)
        .map(|f| ThresholdViolation {
            name: f.name.clone(),
            value: f.line_count,
            threshold,
            kind: ViolationKind::LargeFunction,
        })
        .collect()
}

/// Check if a file exceeds the size threshold.
pub(crate) fn flag_large_file(
    filename: &str,
    line_count: usize,
    threshold: usize,
) -> Option<ThresholdViolation> {
    if line_count > threshold {
        Some(ThresholdViolation {
            name: filename.to_string(),
            value: line_count,
            threshold,
            kind: ViolationKind::LargeFile,
        })
    } else {
        None
    }
}

/// Information about a single function extracted from the AST.
#[derive(Debug, Clone)]
pub(crate) struct FunctionInfo {
    /// Function name.
    pub(crate) name: String,
    /// Number of lines the function spans.
    pub(crate) line_count: usize,
    /// Cyclomatic complexity of this function.
    pub(crate) cyclomatic: usize,
    /// Cognitive complexity of this function.
    pub(crate) cognitive: usize,
}

/// Extract all functions from source code with their metrics.
pub(crate) fn extract_functions(content: &str, language: Language) -> Option<Vec<FunctionInfo>> {
    let tree = parse(content, language)?;
    let root = tree.root_node();
    let source = content.as_bytes();
    let mut functions = Vec::new();
    collect_functions(root, source, &mut functions);
    Some(functions)
}

/// Recursively collect function info from the AST.
fn collect_functions(node: Node, source: &[u8], out: &mut Vec<FunctionInfo>) {
    if is_function_node(node.kind()) {
        let name = extract_function_name(node, source);
        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        let line_count = end_line - start_line + 1;
        let cyclomatic = 1 + count_decision_points(node, source);
        let cognitive = cognitive_score(node, 0);

        out.push(FunctionInfo {
            name,
            line_count,
            cyclomatic,
            cognitive,
        });
        return;
    }

    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            collect_functions(child, source, out);
        }
    }
}

/// Extract the function name from a function node.
fn extract_function_name(node: Node, source: &[u8]) -> String {
    // Look for a "name" or "identifier" child
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "identifier" | "name" | "property_identifier" => {
                    return child.utf8_text(source).unwrap_or("<unknown>").to_string();
                }
                _ => {}
            }
        }
    }
    "<anonymous>".to_string()
}

/// Recursively find functions and compute cognitive complexity.
fn collect_cognitive_complexities(node: Node, total: &mut usize, count: &mut usize) {
    if is_function_node(node.kind()) {
        let score = cognitive_score(node, 0);
        *total += score;
        *count += 1;
        return;
    }

    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            collect_cognitive_complexities(child, total, count);
        }
    }
}

/// Compute cognitive complexity score for a subtree at a given nesting depth.
fn cognitive_score(node: Node, nesting: usize) -> usize {
    let mut score = 0;

    let child_count = node.child_count();
    for i in 0..child_count {
        if let Some(child) = node.child(i) {
            if is_function_node(child.kind()) {
                continue; // Don't recurse into nested functions
            }

            if is_nesting_increment(child.kind()) {
                // +1 for the structure itself, +nesting for depth
                score += 1 + nesting;
                // Recurse into children with increased nesting
                score += cognitive_score(child, nesting + 1);
            } else if is_cognitive_increment(child.kind()) {
                // +1 without nesting (e.g., else, else if)
                score += 1;
                score += cognitive_score(child, nesting);
            } else {
                score += cognitive_score(child, nesting);
            }
        }
    }

    score
}

/// Node kinds that increment cognitive complexity AND increase nesting.
fn is_nesting_increment(kind: &str) -> bool {
    matches!(
        kind,
        "if_expression"
            | "if_statement"
            | "if_let_expression"
            | "while_expression"
            | "while_statement"
            | "while_let_expression"
            | "for_expression"
            | "for_statement"
            | "for_in_statement"
            | "loop_expression"
            | "match_expression"
            | "switch_statement"
            | "catch_clause"
            | "except_clause"
            | "rescue"
            | "ternary_expression"
            | "conditional_expression"
    )
}

/// Node kinds that increment cognitive complexity but DON'T increase nesting.
fn is_cognitive_increment(kind: &str) -> bool {
    matches!(
        kind,
        "else_clause" | "elif_clause" | "else_if_clause" | "boolean_operator"
    )
}

/// Recursively find function nodes and compute their cyclomatic complexity.
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

    // --- Cognitive complexity tests ---

    #[test]
    fn cognitive_simple_function_is_zero() {
        let code = "fn hello() { println!(\"hi\"); }";
        let result = cognitive_complexity(code, Language::Rust).unwrap();
        assert_eq!(result.total, 0, "simple function = cognitive 0");
    }

    #[test]
    fn cognitive_single_if_is_one() {
        let code = "fn check(x: i32) { if x > 0 { return; } }";
        let result = cognitive_complexity(code, Language::Rust).unwrap();
        assert!(
            result.total >= 1,
            "single if = at least cognitive 1, got {}",
            result.total
        );
    }

    #[test]
    fn cognitive_nested_if_adds_nesting_penalty() {
        let code = r#"
fn check(x: i32, y: i32) {
    if x > 0 {
        if y > 0 {
            return;
        }
    }
}
"#;
        let result = cognitive_complexity(code, Language::Rust).unwrap();
        // outer if = +1 (nesting 0), inner if = +1+1 (nesting 1) = total 3
        assert!(
            result.total >= 3,
            "nested if should have cognitive >= 3, got {}",
            result.total
        );
    }

    #[test]
    fn cognitive_unsupported_returns_none() {
        let result = cognitive_complexity("SELECT 1;", Language::SQL);
        assert!(result.is_none());
    }

    #[test]
    fn cognitive_python_nested() {
        let code = "def check(x, y):\n    if x > 0:\n        if y > 0:\n            return True\n    return False\n";
        let result = cognitive_complexity(code, Language::Python).unwrap();
        assert!(
            result.total >= 3,
            "nested Python ifs should have cognitive >= 3, got {}",
            result.total
        );
    }

    // --- Function extraction tests ---

    #[test]
    fn extracts_rust_functions() {
        let code = r#"
fn hello() { println!("hi"); }
fn world() {
    if true {
        return;
    }
}
"#;
        let funcs = extract_functions(code, Language::Rust).unwrap();
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].name, "hello");
        assert_eq!(funcs[1].name, "world");
    }

    #[test]
    fn function_has_line_count() {
        let code = "fn multi() {\n    let a = 1;\n    let b = 2;\n    let c = 3;\n}\n";
        let funcs = extract_functions(code, Language::Rust).unwrap();
        assert_eq!(funcs.len(), 1);
        assert!(
            funcs[0].line_count >= 4,
            "multi-line function should have 4+ lines, got {}",
            funcs[0].line_count
        );
    }

    #[test]
    fn function_has_complexity_scores() {
        let code = "fn check(x: i32) { if x > 0 { return; } }";
        let funcs = extract_functions(code, Language::Rust).unwrap();
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].cyclomatic, 2);
        assert!(funcs[0].cognitive >= 1);
    }

    #[test]
    fn extracts_python_functions() {
        let code = "def hello():\n    pass\ndef world():\n    pass\n";
        let funcs = extract_functions(code, Language::Python).unwrap();
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].name, "hello");
        assert_eq!(funcs[1].name, "world");
    }

    #[test]
    fn extract_returns_none_for_unsupported() {
        let result = extract_functions("SELECT 1;", Language::SQL);
        assert!(result.is_none());
    }

    // --- Threshold flagging tests ---

    #[test]
    fn flags_large_function() {
        let functions = vec![
            FunctionInfo {
                name: "small".to_string(),
                line_count: 10,
                cyclomatic: 1,
                cognitive: 0,
            },
            FunctionInfo {
                name: "big".to_string(),
                line_count: 60,
                cyclomatic: 5,
                cognitive: 10,
            },
        ];
        let violations = flag_large_functions(&functions, 50);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "big");
        assert_eq!(violations[0].value, 60);
    }

    #[test]
    fn no_violations_when_under_threshold() {
        let functions = vec![FunctionInfo {
            name: "ok".to_string(),
            line_count: 30,
            cyclomatic: 2,
            cognitive: 1,
        }];
        let violations = flag_large_functions(&functions, 50);
        assert!(violations.is_empty());
    }

    #[test]
    fn flags_large_file() {
        let v = flag_large_file("big.rs", 600, 500);
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.value, 600);
    }

    #[test]
    fn no_flag_for_small_file() {
        let v = flag_large_file("small.rs", 100, 500);
        assert!(v.is_none());
    }
}
