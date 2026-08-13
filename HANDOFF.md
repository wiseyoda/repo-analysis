# Repostat Suite Integration Handoff

## Installed release — 2026-08-12

Repostat 0.9.0 is published on `main` and active from immutable package
`b9589e8b5c6734acdee2a5f00f650306f6a300ff648e8e160d0f72a96478cbe0`.
Suite doctor is green. The installed Engine resolves this package without a
source checkout or caller-selected executable.

Installed root `019ff8d8-4b9e-7000-8209-1348bce0c539` completed the exact
eight-event terminal sequence. Artifact
`019ff8d8-4dff-7000-ac2e-5e3703867fed` was read back through the canonical API:
17,668 bytes, SHA-256
`9ab81fded71bb155d2a4b5cc52934d7b7d16fcfa5137bdcdc4718df8ae27d1dc`.
The target repository status fingerprint was identical before and after.

## Historical immutable-runtime checkpoint — 2026-07-27

- Repostat commit `62e7aa4` adds a closed runtime-staging command. Its exact
  full gate now runs rustfmt, Clippy with warnings denied, 228 unit tests, 15
  CLI tests, 2 runtime-staging tests, 3 generated-protocol tests, 10 suite-
  conformance tests, and a locked release build.
- `scripts/prepare-suite-runtime.sh` publishes exactly the release binary,
  extension manifest, and result schema through a private sibling staging
  directory and atomic rename. It normalizes the binary to `0500`, data files
  to `0400`, refuses replacement, and performs no implicit build, install,
  signing, provider, account, network, or source mutation.
- The resulting binary SHA-256 is
  `3933688418d6e28cced0430914d3a868d96b6cebb6e48ca0717f75465800ff49`.
  Engine built isolated package
  `7a4e77d30192429979da2fc059778fa78af3d61dc141edbb9ddd70472dc12850`;
  catalog and doctor reported it active, compatible, and healthy.
- Engine commit `851f4d0` removes caller-selected Repostat installation
  references. Engine resolves and re-inspects the active trusted package,
  rejects caller-selected code, and pins both root and child runs to its exact
  package digest.
- A real isolated Engine tool run from that package succeeded and captured a
  17,668-byte immutable artifact with SHA-256
  `4afc98ec95403f81368a58f9d384cddb309aeaf0c7e2a98c4ea39cfa15fe11af`,
  byte-identical to standalone `--json --no-write` output. Engine's exact gate
  passes 1,725 tests with 4,397 assertions across 162 files plus all static
  checks.
- Every install and run in this checkpoint used disposable
  `LVL_TEST_GLOBAL_DIR` state. No host suite installation, provider/account
  call, publish, push, deployment, signing, or target-repository mutation
  occurred. Host RC, sandbox, restart, source-removal, CI artifact retention,
  upgrade, and rollback proof remain.

Updated: 2026-07-26

Branch: `codex/suite-integration`

Starting baseline: `59323ea`

Protocol V1 RC consumer: `ca5fb96`

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
- The manifest pins Protocol V1 `1.0.0-rc.1` at aggregate SHA-256
  `bb33b84968522aad60993c3459d67a45bd82e03ddc86e30d4d56238af2b80b5a`.
  Generated Rust types, structural decoders, schema descriptors, release
  records, and the versioned fake tool/workflow corpus are vendored so Repostat
  does not import an Engine checkout.

## Verification

- Full `./scripts/verify.sh`: passed.
  - rustfmt check: passed
  - Clippy with warnings denied: passed
  - unit tests: 228 passed, 0 failed
  - legacy CLI integration: 15 passed, 0 failed
  - generated Protocol V1 tests: 3 passed, 0 failed
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

- Engine commit `f817c45` validates the manifest and result contract, invokes
  the adapter through its durable supervisor, and captures the exact stdout as
  a checksummed immutable artifact.
- A real temporary-package proof produced byte-identical standalone and Engine
  artifacts: SHA-256
  `32fadbe84eefb6f95ea8dbc0360c7622f1d5127b9e92b4e7bd59ea74104b43a7`,
  2,212 bytes.
- Engine validates the declared `repository.read` permission and detects target
  mutation, but OS-level runtime sandbox enforcement remains incomplete.
- Standalone and Engine invocation must pass parity from immutable installed
  release-candidate artifacts.
- Publishing, pushing, installation, and live-provider work remain unapproved.

RPS-01 and RPS-02 are local Candidates. Neither is Accepted or shipped until
immutable installed release-candidate parity and runtime sandbox evidence pass.
