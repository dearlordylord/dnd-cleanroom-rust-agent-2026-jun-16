# Merge Queue

Campaign: `level-1-2-runtime-reducer-route`

This file records lane branches that are implemented, reviewed, and waiting to merge into the integration worktree.

## Current Queue

1. `L15-RR09-CHARACTER-SHEET-ROUTES`
   - Lane head: `410e3db7d181c7fba9265cc753ab977198dacca7`
   - Worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09`
   - Review: clean after Aristotle fixer and Jason re-review.
   - Merge note: RR08 is merged; run explicit conflict review for `src/tests/mod.rs`, task artifacts, and `src/qnt_adapters/character_sheet_weapon_mastery_containers_selected_identity.rs`.

## Merge Rules

- Merge only into `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19` on branch `ralph/rrconv-19-cleanroom`.
- Use non-fast-forward merges with a message of the form:
  - `Merge <lane id> cleanroom <short description>`
- After each merge, run:
  - `cargo fmt --check`
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `node scripts/check-cleanroom-harness.cjs`
  - `git diff --check HEAD~1...HEAD`
- Update `STATE.json`, `CHECKPOINT_REPORT.md`, and `WORKTREE_LEDGER.md` after the merge result is known.

## Completed Before Campaign Bootstrap

- `RRCONV-19E` merged at `aa3996e4b32e2502b3c3ee5cca051db66157269b`.
- `RRCONV-19F` merged at `be336582921801cd06995121db38e34ca6f4e275`.
- `RRCONV-19G` merged at `6e3ec7c4fff70a28a4ab29cfebeaf9133daec4f0`.
