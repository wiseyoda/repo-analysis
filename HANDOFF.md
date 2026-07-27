# Repostat Suite Integration Handoff

Updated: 2026-07-26

Branch: `codex/suite-integration`

Starting baseline: `59323ea`

## Scope

RPS-01 and the Repostat-owned portion of RPS-02 make Repostat the suite's
deterministic tool provider. The controlling decision is ADR-008 in
`docs/decisions.md`; the executable contract is
`docs/specs/deterministic-suite-tool.md`.

## Implemented

- Default analysis, JSON, Markdown, and `repostat extension` start no model or
  provider and write no target or global state.
- `--save` is the explicit snapshot/index write; `--html` remains an explicit
  target export; `--no-write` rejects both write modes.
- Standalone `--json --no-write` and `repostat extension` use the same scanner
  and emit byte-identical, timestamp-free `repostat.metrics.v1` JSON.
- Output identifies the canonical target and its Git SHA, uses deterministic
  ordering, ignores user-global Git excludes, and computes churn over all
  reachable history.
- `ai-mux.extension.json` declares only `repository.read`, no Engine privileged
  capability, and the `repostat.metrics.v1` artifact type.
- The closed schema is
  `schemas/repostat-metrics-v1.schema.json`.
- Direct Claude execution and skill orchestration were removed. Historical AI
  snapshot fields remain readable.
- Package, repository, Homebrew, contributor, architecture, and operator
  documentation now describe the current boundary.

## Verification

- Full `./scripts/verify.sh`: passed.
  - rustfmt check: passed
  - Clippy with warnings denied: passed
  - unit tests: 228 passed, 0 failed
  - legacy CLI integration: 15 passed, 0 failed
  - suite conformance: 10 passed, 0 failed
- Additional Clippy run over all targets with warnings denied: passed.

The suite tests cover provider mutation traps, recursive before/after
fingerprints, explicit writes, repeated-byte equality, standalone/extension
parity, target Git identity, global Git-ignore isolation, closed
manifest/schema fields, and distribution metadata.

## Preserved Pre-existing Work

The untracked `.code-review-agent/` directory and `REVIEW.md` predate this
integration branch. They are intentionally neither modified nor staged.

## Remaining Cross-repository Evidence

- Engine must invoke the installed adapter through its supervisor and capture
  stdout as a checksummed immutable artifact.
- Engine must validate the manifest/result contract and enforce
  `repository.read` at runtime.
- Standalone and Engine invocation must pass parity from immutable installed
  release-candidate artifacts.
- Publishing, pushing, installation, and live-provider work remain unapproved.

Until those items pass, RPS-01 is a local implementation candidate and RPS-02
is only partially satisfied; neither is shipped.
