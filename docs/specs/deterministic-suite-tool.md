# Deterministic Suite Tool

**Requirement IDs:** R-1000 through R-1012

**Suite initiatives:** RPS-01, RPS-02

**Decision:** ai-mux D-03, accepted 2026-07-26

**Status:** Accepted for implementation

## Purpose

Repostat is the suite's deterministic repository-metrics tool. An ordinary
scan must be fast, token-free, and read-only. Engine must be able to invoke the
same scanner as a tool and store the returned bytes as an Engine-owned
`repostat.metrics.v1` artifact without Repostat choosing a target-repository
output path.

Historical snapshots, trends, the repository index, and explicit HTML export
remain useful standalone features. They become explicit writes rather than
unavoidable effects of analysis.

AI enrichment is no longer a Repostat subprocess responsibility. A future
Engine workflow may add architecture, feature, quality, or effort results
through typed child agent requests. Existing snapshots containing historical
`ai_analysis` data remain readable.

## Required Behavior

| ID | Requirement | Priority |
|---|---|---|
| R-1000 | A default scan never invokes Claude, `lvl`, or another provider/model process | P0 |
| R-1001 | A default scan does not write inside the target repository or Repostat's global data directory | P0 |
| R-1002 | `--save` explicitly writes a timestamped snapshot and updates the cross-repository index | P0 |
| R-1003 | `--no-write` explicitly guarantees no snapshot, index, or HTML write and conflicts with write flags | P0 |
| R-1004 | `--json` emits a stable `repostat.metrics.v1` object without timestamps, artifact IDs, or AI content | P0 |
| R-1005 | The structured result uses the canonical target root and target repository Git SHA | P0 |
| R-1006 | Repeated scans of unchanged input emit byte-identical JSON | P0 |
| R-1007 | `repostat extension [path]` emits only the same structured JSON on stdout and performs no writes | P0 |
| R-1008 | Standalone `--json --no-write` and `extension` results are byte-identical for the same canonical root | P0 |
| R-1009 | The repository ships a closed JSON Schema and suite extension manifest for `repostat.metrics.v1` | P0 |
| R-1010 | Risk/churn inputs do not depend on wall-clock time | P1 |
| R-1011 | User-global Git ignore configuration cannot change scanner results | P1 |
| R-1012 | Cargo, README, and Homebrew metadata use the current `wiseyoda/ai-mux-repostat` repository | P1 |
| R-1013 | The repository vendors the generated Protocol V1 RC Rust types, structural decoders, schema descriptors, and aggregate hash from the canonical Engine schema package | P0 |
| R-1014 | A repository-owned command stages one closed release runtime root | P0 |
| R-1015 | Staging requires an existing executable binary and has no implicit build, install, signing, publish, or source-write side effect | P0 |
| R-1016 | Staging normalizes executable and data modes before Engine hashing | P0 |
| R-1017 | Staging refuses replacement and publishes only by atomic rename | P0 |
| R-1018 | Engine remains the authority for package provenance, hashing, trust, installation, lifecycle, and run binding | P0 |

## CLI Contract

```text
repostat [--json|--markdown|--html] [--save] [--no-write] [path]
repostat extension [path]
```

- No write flag: deterministic scan; stdout/dashboard only.
- `--save`: persist the normal historical snapshot and update the global index.
- `--no-write`: document and enforce the read-only contract. It conflicts with
  `--save` and `--html`.
- `--html`: explicit legacy export to `repostat-report.html`; it does not imply
  `--save`.
- `extension`: thin Engine tool adapter. It accepts no history or export flags,
  emits protocol payload bytes only on stdout, and never starts a model.

## Structured Result

The `repostat.metrics.v1` result is closed and contains:

- schema and artifact-type identifiers;
- canonical source root and target repository Git SHA;
- total and per-language line/file metrics;
- deterministic complexity hotspots;
- dependency metrics;
- documentation metrics;
- skipped-file count;
- deterministic risk inputs and scores.

It intentionally excludes:

- generation timestamps;
- Engine run, artifact, or correlation IDs;
- output paths;
- AI/model results;
- snapshot history or deltas.

Engine owns run metadata, timestamps, artifact IDs, retention, checksums, and
artifact paths.

## Release Runtime Staging

Repostat produces a prepared runtime root before Engine constructs a suite
package:

```text
repostat
ai-mux.extension.json
schemas/repostat-metrics-v1.schema.json
```

`scripts/prepare-suite-runtime.sh <destination>` stages the existing
`target/release/repostat` binary. `--binary <path>` selects an explicit
cross-compiled or CI-produced binary. The command:

- requires the binary to exist, be a regular file, and be executable;
- refuses an existing destination;
- copies through a private sibling staging directory;
- normalizes the executable to mode `0500` and the two data files to `0400`;
- atomically renames the complete staging directory to the destination; and
- performs no build, install, signing, provider, account, network, or source
  mutation.

The prepared root is not an installed package and carries no independent trust
claim. Engine adds the declared source repository and commit, exact file hashes
and modes, trust digest, compatibility checks, lifecycle records, and run
binding when it constructs and installs the immutable suite package.

## Compatibility

- Existing snapshot files and AI fields remain deserializable.
- `trend` and `list` continue to read historical state.
- Existing dashboard and Markdown output remain available.
- The former `REPOSTAT_SKIP_AI` switch is obsolete because deterministic
  analysis never invokes AI.
- Direct Claude/skill orchestration is removed from the compiled product.
- The extension manifest pins the Protocol V1 RC aggregate schema hash. The
  vendored Rust helper must decode all canonical examples, reject undeclared
  fields, and match that same hash without importing an Engine checkout.

## Acceptance Evidence

- Unit tests prove result ordering, canonical roots, target Git SHA, and
  time-independent churn.
- CLI conformance runs provider-name mutation traps and observes no invocation.
- A before/after recursive target and global-data fingerprint is identical for
  default, JSON no-write, and extension scans.
- Two standalone results and one extension result are byte-identical.
- `--save` alone creates snapshot/index state.
- Manifest and result examples validate against their closed schemas.
- Vendored Rust helpers compile in this independent repository, decode all
  eight canonical examples, reject structural drift, and match the manifest
  protocol hash.
- Runtime-staging tests prove the exact file set, byte preservation, normalized
  modes, existing-destination refusal, and missing/non-executable binary errors.
- `cargo fmt --check`, Clippy with warnings denied, and the full Rust test suite
  pass.
