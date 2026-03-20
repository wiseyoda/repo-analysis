# Architectural Decision Records

> Each decision is numbered, dated, and immutable once accepted. Superseded decisions
> are marked but never deleted — they provide historical context.
>
> Format: ADR-NNN | Status | Date

---

## ADR-001 | Accepted | 2026-03-19

### Use Rust as the implementation language

**Context**: We need a CLI tool that is fast, distributable as a single binary, and reliable.
Candidates: Rust, Go, TypeScript (Bun), hybrid Rust+TS.

**Decision**: Pure Rust.

**Rationale**:
- Single binary with zero runtime dependencies
- Tree-sitter has first-class Rust bindings
- `rayon` provides trivial parallelism
- Strong type system catches bugs at compile time
- The `ignore` crate (ripgrep ecosystem) provides battle-tested gitignore handling
- Performance is native-speed for file I/O and parsing

**Trade-offs**:
- Longer development time vs TypeScript
- Steeper learning curve for contributors
- Compile times slower than Go

---

## ADR-002 | Accepted | 2026-03-19

### Use tree-sitter for source code parsing

**Context**: Complexity analysis (cyclomatic, cognitive) requires understanding code structure.
Options: tree-sitter, regex heuristics, language-specific AST tools.

**Decision**: Tree-sitter with compiled grammars for top 10 languages, dynamic loading for extras.

**Rationale**:
- One parser framework for all languages
- Incremental, error-tolerant parsing
- Grammars available for 100+ languages
- Used by GitHub for syntax highlighting — well maintained
- Rust bindings are first-class

**Trade-offs**:
- Grammar files add to binary size (~2-5MB per language)
- Some grammars have quirks requiring workarounds
- Dynamic grammar loading requires network on first use for non-bundled languages

---

## ADR-003 | Accepted | 2026-03-19

### Use Claude CLI for AI-augmented analysis

**Context**: We want LLM-powered architecture summaries, feature inventories, quality reviews,
and effort estimation. Options: direct API calls, Claude CLI, multi-provider.

**Decision**: Invoke Claude CLI (`claude -p`) as a subprocess in the target directory.

**Rationale**:
- Claude CLI runs in the target directory and has full codebase access via its own file tools
- No need to chunk or pass files — Claude CLI handles context management
- Skill files provide structured, versioned prompt templates
- JSON output mode gives structured responses
- User already has Claude CLI installed and authenticated
- Graceful degradation when CLI is unavailable

**Trade-offs**:
- Depends on external binary being installed
- Less control over token usage than direct API
- Subprocess invocation adds overhead vs in-process API calls

---

## ADR-004 | Accepted | 2026-03-19

### Store snapshots as individual JSON files

**Context**: Need to persist analysis results for historical comparison. Options: SQLite, JSON
files, git-native reconstruction.

**Decision**: Individual JSON files in `.repostat/snapshots/`, named by timestamp.

**Rationale**:
- Zero additional dependencies (serde_json already required)
- Human-readable, inspectable, diffable
- Git-friendly (each snapshot is a new file, no merge conflicts)
- Simple to implement, easy to debug
- Cross-repo index in `~/.repostat/repos.json` for listing

**Trade-offs**:
- Querying across many snapshots requires loading each file
- Not as efficient as SQLite for trend analysis over 100+ snapshots
- Can revisit with SQLite if performance becomes an issue (see BACKLOG)

---

## ADR-005 | Accepted | 2026-03-19

### Layered exclusion system

**Context**: Must distinguish user-written code from vendored/generated code. Different repos
have different conventions.

**Decision**: Three-layer exclusion:
1. `.gitignore` (via `ignore` crate)
2. Built-in heuristics (known dirs, minification detection, generated file headers)
3. `.repostat.toml` user overrides

**Rationale**:
- Layer 1 covers most cases automatically
- Layer 2 catches what gitignore misses (committed vendor code, generated files)
- Layer 3 gives users full control
- Layers compose: each is additive, user config can also force-include

**Trade-offs**:
- Three-layer system is more complex to implement and test
- Edge cases in layer interaction need clear precedence rules

---

## ADR-006 | Accepted | 2026-03-19

### Skill files for AI prompts

**Context**: AI analysis prompts need to be structured, versioned, and customizable.

**Decision**: Store prompts as skill files in `~/.repostat/skills/`. The tool ships with
defaults that are written on first run if the directory is empty.

**Rationale**:
- Users can customize analysis prompts without rebuilding
- Skill files can be version-controlled separately
- New analysis types can be added by dropping in a new skill file
- Clear separation between tool logic and prompt engineering

**Trade-offs**:
- External file dependency (mitigated by writing defaults on first run)
- Users could break analysis by editing skill files incorrectly

---

## ADR-007 | Accepted | 2026-03-19

### Fast model always for AI analysis

**Context**: LLM analysis is the slowest part of the pipeline. Options: fast model always,
smart model opt-in, tiered, configurable.

**Decision**: Always use a fast model (Haiku-class). Speed over nuance.

**Rationale**:
- Primary use case is progress tracking, not deep code review
- Fast model keeps total analysis time under 30 seconds
- Summaries and inventories don't require frontier-model reasoning
- Cost is negligible with fast models

**Trade-offs**:
- Less nuanced architecture and quality analysis
- May miss subtle patterns that larger models would catch
- Can revisit with `--model` flag if users want deeper analysis (see BACKLOG)

---

## Template for new decisions

```markdown
## ADR-NNN | Proposed | YYYY-MM-DD

### Title

**Context**: What is the issue?

**Decision**: What did we decide?

**Rationale**: Why this choice?

**Trade-offs**: What are we giving up?
```
