# AI-Augmented Analysis — Feature Spec

> Phase 5 of the repostat roadmap. Claude CLI integration for architecture,
> features, quality, and effort estimation.

## Purpose

Add AI-powered analysis to repostat by invoking the Claude CLI as a subprocess.
Skill files define analysis prompts. Results are parsed, stored in snapshots,
and displayed in the dashboard. The tool gracefully skips AI analysis when
the Claude CLI is unavailable.

## Requirements

| ID | Description | Priority |
|----|-------------|----------|
| R-400 | Detect Claude CLI availability with graceful skip if missing | P0 |
| R-401 | Skill file system: load analysis prompts from `~/.repostat/skills/` | P0 |
| R-402 | Invoke `claude -p` in target directory with skill content and JSON output | P0 |
| R-403 | Lenient JSON response parsing: extract what's available, defaults for missing | P0 |
| R-404 | Architecture summary skill | P0 |
| R-405 | Feature inventory skill | P0 |
| R-406 | Code quality review skill | P1 |
| R-407 | Effort estimation skill | P1 |
| R-408 | Store AI analysis results in snapshots | P0 |
| R-409 | AI results displayed in dashboard | P0 |

## Architecture

Per ADR-003 and ADR-006:

```
src/ai/
  mod.rs       — AI orchestration: detect CLI, run all skills, collect results
  claude.rs    — Claude CLI invocation: subprocess management, output capture
  skills.rs    — Skill file loading: read from disk, write defaults
  schema.rs    — Response parsing: lenient JSON extraction with typed defaults
```

## Data Model

### CliStatus

```rust
enum CliStatus {
    Available(PathBuf),  // path to claude binary
    Unavailable,         // not found
}
```

### SkillFile

```rust
struct SkillFile {
    name: String,       // e.g. "architecture"
    prompt: String,     // the full prompt content
}
```

### AiAnalysisResult

Aggregate result from all skill invocations.

```rust
struct AiAnalysisResult {
    architecture: Option<ArchitectureSummary>,
    features: Option<FeatureInventory>,
    quality: Option<QualityReview>,
    effort: Option<EffortEstimate>,
    stale_docs: Option<StaleDocs>,
    doc_quality: Option<DocQuality>,
}
```

### Per-skill response types

```rust
struct ArchitectureSummary {
    description: String,
    patterns: Vec<String>,
    design_approach: String,
}

struct FeatureInventory {
    features: Vec<Feature>,
}

struct Feature {
    name: String,
    status: String,       // "complete", "wip", "planned"
    description: String,
}

struct QualityReview {
    issues: Vec<QualityIssue>,
    overall_score: String,  // "good", "fair", "poor"
}

struct QualityIssue {
    category: String,     // "anti-pattern", "dead-code", "inconsistency"
    description: String,
    file: Option<String>,
}

struct EffortEstimate {
    existing_hours: f64,
    remaining_hours: Option<f64>,
    summary: String,
}

struct StaleDocs {
    stale_files: Vec<StaleDocEntry>,
}

struct StaleDocEntry {
    file: String,
    reason: String,
}

struct DocQuality {
    overall_score: String,
    files: Vec<DocQualityEntry>,
}

struct DocQualityEntry {
    file: String,
    score: String,
    feedback: String,
}
```

## Behavior

### CLI Detection (R-400)

1. Run `which claude` (Unix) or `where claude` (Windows).
2. If found, return `CliStatus::Available(path)`.
3. If not found, return `CliStatus::Unavailable`.
4. When unavailable, skip all AI analysis silently.

### Skill File System (R-401)

1. On first run, check if `~/.repostat/skills/` exists.
2. If not, create it and write 6 default skill files.
3. Skill files are plain text with the analysis prompt.
4. Load all `.md` files from the skills directory.

Default skill files:
- `architecture.md` — architecture summary prompt
- `features.md` — feature inventory prompt
- `quality.md` — code quality review prompt
- `effort.md` — effort estimation prompt
- `stale-docs.md` — stale documentation detection prompt
- `doc-quality.md` — documentation quality scoring prompt

### Claude CLI Invocation (R-402)

1. For each skill file, invoke:
   ```
   claude -p "<skill prompt>" --output-format json
   ```
   in the target directory.
2. Capture stdout as the JSON response.
3. Timeout after 60 seconds per skill.
4. If a skill invocation fails, log to stderr and continue with others.

Per ADR-007, always use the default (fast) model — no `--model` flag needed.

### Response Parsing (R-403)

1. Attempt to parse stdout as JSON.
2. If JSON parsing fails, attempt to extract JSON from markdown code blocks.
3. For each expected field, use a default if missing.
4. Never fail the entire analysis because one field is missing.

### Graceful Degradation (R-400)

When Claude CLI is unavailable:
- No error messages printed
- AI section omitted from dashboard
- Snapshot `ai_analysis` field is `null`
- All other analysis continues normally

## Dashboard Section

```
├────────────────────────────────────────┤
│ AI Analysis                            │
│ ─────────────────────────────────────  │
│  Architecture: Web API with layered    │
│    design                              │
│  Features: 12 complete, 3 WIP          │
│  Quality: Good (2 issues)              │
│  Effort: ~240 dev-hours                │
└────────────────────────────────────────┘
```

## Snapshot Schema

```json
{
  "ai_analysis": {
    "architecture": {
      "description": "...",
      "patterns": ["MVC", "Repository"],
      "design_approach": "layered"
    },
    "features": {
      "features": [
        {"name": "Auth", "status": "complete", "description": "..."}
      ]
    },
    "quality": {
      "issues": [...],
      "overall_score": "good"
    },
    "effort": {
      "existing_hours": 240.0,
      "remaining_hours": 80.0,
      "summary": "..."
    }
  }
}
```

## Edge Cases

- Claude CLI installed but not authenticated: invocation will fail; treat as unavailable for that skill.
- Claude CLI returns non-JSON output: extract JSON from markdown code blocks.
- Claude CLI returns partial JSON: use defaults for missing fields.
- Skill file is empty or corrupted: skip that skill, continue with others.
- `~/.repostat/skills/` is not writable: use built-in defaults, warn to stderr.
- Network timeout: 60s per skill, skip on timeout.

## Acceptance Criteria

- [ ] `which claude` detection works on macOS/Linux.
- [ ] Default skill files are written to `~/.repostat/skills/` on first run.
- [ ] Skill files are loaded and passed to Claude CLI.
- [ ] JSON responses are parsed with defaults for missing fields.
- [ ] Dashboard shows AI section when results are available.
- [ ] Dashboard omits AI section when Claude CLI is missing.
- [ ] Snapshot includes `ai_analysis` field.
- [ ] Old snapshots without `ai_analysis` deserialize correctly.
