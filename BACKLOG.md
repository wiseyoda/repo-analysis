# Backlog

> Deferred items that are valuable but not on the current roadmap.
> Items are moved here from active phases when scope is cut, or added when
> new ideas arise during development.
>
> Format: Brief description, origin (which phase/discussion deferred it), and
> conditions under which it should be promoted to the roadmap.

---

## Deferred from Requirements Gathering

### SQLite snapshot storage
**Origin**: ADR-004 trade-offs.
**Description**: Replace or supplement JSON file snapshots with a SQLite database for
efficient querying across many snapshots.
**Promote when**: Users accumulate 100+ snapshots and querying becomes slow.

### Direct Anthropic API integration
**Origin**: ADR-003 trade-offs.
**Description**: Call the Anthropic API directly instead of shelling out to Claude CLI.
Gives more control over token budgets, model selection, and streaming.
**Promote when**: Claude CLI becomes a bottleneck or users need fine-grained model control.

### Multi-provider AI support
**Origin**: Round 4 Q3 alternatives.
**Description**: Support Gemini, OpenAI, and other providers alongside Claude.
Config file picks the provider.
**Promote when**: User demand or if a provider offers better price/performance for
summary-level analysis.

### `--model` flag for AI analysis
**Origin**: ADR-007 trade-offs.
**Description**: Allow users to specify which model to use for AI analysis
(e.g., `--model sonnet` for deeper analysis).
**Promote when**: Users report that fast-model analysis is too shallow.

### Remote repo support
**Origin**: Round 1 Q4 — CLI invocation.
**Description**: `repostat gh:org/repo` clones/fetches a remote repo and analyzes it.
**Promote when**: Phase 7 (distribution) is complete and local analysis is solid.

### CI integration mode
**Origin**: Round 1 Q4 — CLI invocation.
**Description**: Run repostat in GitHub Actions with PR comments showing metric deltas.
**Promote when**: Core analysis is stable and users request CI integration.

### Dependency security + staleness
**Origin**: Round 1 Q3 — deps scope.
**Description**: Check dependencies for known vulnerabilities (via advisory databases)
and flag unmaintained packages (no release in 12+ months).
**Promote when**: Phase 3 (dependency analysis) is complete.

### API surface analysis
**Origin**: Round 1 Q3 — deps scope.
**Description**: Analyze how many exports from each dependency the code actually uses.
Detect unused dependencies.
**Promote when**: Phase 3 internal coupling analysis is complete.

### Incremental analysis mode
**Origin**: Round 8 Q1 — performance.
**Description**: Only re-analyze files changed since last snapshot (using mtime or git diff).
`--incremental` flag.
**Promote when**: Full scan exceeds 10 seconds on target repos.

### HTML dashboard output
**Origin**: Round 2 Q2 — report format.
**Description**: Self-contained HTML file with interactive charts and drill-downs.
**Promote when**: Markdown reports feel limiting and users want richer visualization.

---

## Ideas (Not Yet Scoped)

- **CODEOWNERS analysis**: Map complexity hotspots to code owners
- **Churn analysis**: Files that change most frequently + are most complex = highest risk
- **Test coverage integration**: Read coverage reports and correlate with complexity
- **Monorepo support**: Analyze sub-packages independently with a unified report
- **Watch mode**: `repostat watch` re-runs on file changes (for live development feedback)
- **Diff mode**: `repostat diff HEAD~10` analyzes only what changed in recent commits
- **Export to CSV**: For teams that want to import metrics into spreadsheets
- **Badges**: Generate SVG badges (complexity score, LOC, etc.) for README
