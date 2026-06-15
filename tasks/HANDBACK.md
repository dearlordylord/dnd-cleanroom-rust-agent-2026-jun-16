# Handback Contract

Use this file when the implementer hands work to a reviewer or decider.

## Required Artifact List

- `tasks/START_GATE.json`
- `tasks/ENGINE_DEPTH_MANIFEST.json`
- `tasks/STATE_OWNER_MANIFEST.json`
- `tasks/target-replay-evidence/*.json`
- `tasks/VALIDATION_REPORT.md`
- `tasks/BLOCKERS.md`
- changed files under `src/**`

## Handback Summary Shape

```md
## <task id>

- Start HEAD: `<start HEAD SHA>`
- Current HEAD: `<current HEAD SHA>`
- Selected drivers:
  - `<cleanroom-input/qnt/.../*.mbt.qnt>`
- Production modules extended:
  - `<src/...>`
- Adapter modules touched:
  - `<src/...>`
- Target replay evidence:
  - `tasks/target-replay-evidence/<file>.json`
- Verification:
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `node scripts/check-cleanroom-harness.cjs`
- Blockers:
  - `_none_` or `<blocker id>`
```

The summary does not replace machine-readable manifests. If the summary and
manifests disagree, the manifests and deterministic checker win.
