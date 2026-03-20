# History & Trends — Feature Spec

> Phase 6 of the repostat roadmap. Sparkline trends, git history integration,
> cross-repo tracking.

## Purpose

Add historical analysis to repostat: visualize metric trends across snapshots,
integrate git log data, compare snapshots, and track multiple repos.

## Requirements

| ID | Description | Priority |
|----|-------------|----------|
| R-500 | `repostat trend` subcommand: sparkline charts across all snapshots | P0 |
| R-501 | Git-aware history: lines added/removed, commit frequency, contributor count | P0 |
| R-502 | Snapshot comparison: diff any two snapshots by timestamp or git SHA | P1 |
| R-503 | Cross-repo index in `~/.repostat/repos.json` | P1 |
| R-504 | `repostat list` subcommand: show all tracked repos | P1 |

## CLI Changes

Refactor from flat flags to subcommands:

```
repostat [path]              → analyze (default, current behavior)
repostat trend [path]        → show sparkline trends across snapshots
repostat list                → show all tracked repos
```

The default (no subcommand) behavior is preserved for backward compatibility.

## Data Model

### GitHistory

```rust
struct GitHistory {
    total_commits: usize,
    contributors: Vec<String>,
    recent_activity: Vec<PeriodStats>,  // last 12 weeks/months
}

struct PeriodStats {
    period: String,         // "2024-W03" or "2024-03"
    commits: usize,
    lines_added: usize,
    lines_removed: usize,
}
```

### RepoIndex

```rust
struct RepoIndex {
    repos: Vec<RepoEntry>,
}

struct RepoEntry {
    path: String,
    name: String,
    last_analyzed: DateTime<Utc>,
    snapshot_count: usize,
}
```

## Behavior

### Trend Subcommand (R-500)

1. Load all snapshots from `.repostat/snapshots/` sorted by timestamp.
2. For each key metric, extract the value across snapshots.
3. Render sparkline charts using Unicode block characters: ▁▂▃▄▅▆▇█

Key metrics for sparklines:
- Total lines of code
- Total files
- Number of languages

Output format:
```
repostat trend — 8 snapshots (2024-01-15 → 2024-03-20)
───────────────────────────────────────────────
  Lines of code  ▁▂▃▃▄▅▆█  1,200 → 6,400
  Files          ▁▂▂▃▃▅▆█     12 →    48
  Languages      ▁▁▂▂▃▃▃▃      2 →     5
───────────────────────────────────────────────
```

### Git History (R-501)

1. Invoke `git log --format="%H|%ae|%ai" --numstat` in the target directory.
2. Parse output to extract:
   - Total commit count
   - Unique contributor emails
   - Per-week stats: commits, lines added, lines removed
3. Display git activity sparklines alongside snapshot trends.

### Snapshot Comparison (R-502)

1. Accept two identifiers: timestamps or git SHAs.
2. Load matching snapshots from store.
3. Compute diff (reuse existing diff logic).
4. Display comparison.

### Cross-Repo Index (R-503/R-504)

1. After each analysis, register the repo in `~/.repostat/repos.json`.
2. `repostat list` reads the index and displays:
   ```
   repostat — tracked repositories
   ───────────────────────────────────
     my-app      /Users/me/my-app       2024-03-20  8 snapshots
     api-server  /Users/me/api-server   2024-03-19  3 snapshots
   ───────────────────────────────────
   ```

### Dashboard Sparklines (R-500)

When 3+ snapshots exist, add inline sparklines to the summary section:

```
│ Lines: 6400       ▁▂▃▄▅▆▇█
│ Files: 48         ▁▂▃▃▅▆▇█
```

## Edge Cases

- No snapshots: trend command prints "No snapshots found."
- Single snapshot: no sparkline (need 2+), show just the values.
- Git not available: skip git history, show only snapshot trends.
- Corrupt snapshot file: skip it, continue with others.
- `~/.repostat/repos.json` doesn't exist: create on first write.

## Acceptance Criteria

- [ ] `repostat trend` shows sparklines for 5+ snapshots.
- [ ] Git history matches `git log --stat` output.
- [ ] `repostat list` shows tracked repos from index.
- [ ] Dashboard shows inline sparklines when snapshots exist.
- [ ] Backward compatible: `repostat [path]` still works.
