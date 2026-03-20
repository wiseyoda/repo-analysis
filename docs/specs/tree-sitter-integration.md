# Spec: Tree-sitter Integration

**Requirement IDs:** R-100
**Phase:** 2
**Date:** 2026-03-19

## Purpose

Integrate tree-sitter for multi-language AST parsing. This is the foundation
for cyclomatic complexity, cognitive complexity, and function extraction —
all of which operate on parsed syntax trees.

## Inputs

| Input | Type |
|-------|------|
| File content | `&str` |
| Language | `Language` |

## Outputs

`Option<tree_sitter::Tree>` — parsed syntax tree, or None if the language
has no tree-sitter grammar.

## Behavior

1. Check if the given Language has a tree-sitter grammar available.
2. Create a `tree_sitter::Parser` and set its language to the appropriate grammar.
3. Parse the content and return the tree.
4. Return None for languages without grammars.

## Supported Languages (Top 10)

TypeScript, JavaScript, Python, Rust, Go, Swift, Java, C, C++, Ruby

## Edge Cases

- Empty content → returns a tree (tree-sitter handles empty input)
- Language without grammar (e.g., SQL, YAML) → returns None
- Invalid/malformed source → tree-sitter is error-tolerant, still returns a tree

## Acceptance Criteria

- [ ] Parses Rust source code and returns a non-empty tree
- [ ] Parses Python source code and returns a non-empty tree
- [ ] Returns None for a language without a grammar
- [ ] All 10 grammars compile and load successfully
- [ ] Existing tests remain green
