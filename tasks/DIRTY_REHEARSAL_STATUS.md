# Dirty Rehearsal Status

This repo has been refreshed from the source cleanroom corpus for all five
`level-1-2-full` lanes.

- Source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Active queue: 96 drivers, 684 replayable in-scope branch obligations
- Existing Rust adapters/evidence before refresh: 74 drivers
- Current handoff evidence: 5 drivers, 24 replayable in-scope branch obligations
- Current evidence status: partial; handoff evidence now references source commit `829aee6441d76a921c9d9c14a0d0221062975334`, while 73 non-handoff existing evidence files still reference source commit `04249edf345a7752de2f1551dd3d509a2fffc160`
- Artifact-model status: stale; this repo has rolling latest artifacts only, with no `tasks/RUN_LEDGER.json` or `tasks/history/<taskId>/*.json`

## Lane Accounting

| Lane | Active Drivers | Newly Active Drivers | Newly Active Obligations |
| --- | ---: | ---: | ---: |
| creation | 8 | 3 | 15 |
| sheet | 10 | 3 | 14 |
| handoff | 5 | 4 | 24 |
| battle | 64 | 9 | 52 |
| rules-core | 9 | 3 | 73 |
| total | 96 | 22 | 178 |

## Newly Active Driver Work

| Lane | Driver | Replayable obligations |
| --- | --- | ---: |
| creation | `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt` | 11 |
| creation | `cleanroom-input/qnt/character-creation-runtime/character-creation-rogue-expertise-selected-identity.mbt.qnt` | 1 |
| creation | `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt` | 3 |
| sheet | `cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt` | 2 |
| sheet | `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt` | 3 |
| sheet | `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt` | 9 |
| handoff | `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt` | 3 |
| handoff | `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt` | 7 |
| handoff | `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt` | 9 |
| handoff | `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt` | 5 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-ability-check-choice-search.mbt.qnt` | 9 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-after-hit-damage-riders.mbt.qnt` | 22 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt` | 4 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt` | 4 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt` | 4 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-casting-time.mbt.qnt` | 1 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-spell-selected-identity.mbt.qnt` | 2 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt` | 3 |
| battle | `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt` | 3 |
| rules-core | `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt` | 31 |
| rules-core | `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt` | 18 |
| rules-core | `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt` | 24 |

## Current Handoff Replays

The dirty rehearsal now has current-snapshot Rust adapters and replay evidence
for every `level-1-2-handoff` driver:

| Driver | In-scope replay evidence |
| --- | ---: |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt` | 2 |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt` | 3 |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt` | 7 |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt` | 9 |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt` | 5 |

Out-of-scope/transit actions remain present in the adapter modules only so the
copied QNT driver shape is visible; they are not claimed as current replay
evidence for the level-1/2 handoff lane.

## Reviewer Notes

- Fixed: `apply_uncanny_metabolism` now takes the Long Rest use-state and
  rejects repeat use through `ResourcePoolError::UncannyMetabolismAlreadyUsed`.
- Fixed: handoff adapter tests replay every copied driver action, including
  transit/out-of-scope actions, while replay evidence remains limited to
  inventory in-scope obligations.
- Rejected: Font of Magic helpers expose the SRD table for slots above level 2.
  Reason: these helpers implement the copied shared rule-core QNT
  `font-of-magic-resource.qnt`; level-1/2 scope is enforced at evidence
  selection, not by narrowing the reusable rule helper.

## Verification

- `cargo fmt --check` passed.
- `cargo test` passed: 155 tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed as expected for this dirty partial refresh:
  - missing `tasks/RUN_LEDGER.json`;
  - no `tasks/history/<taskId>/*.json` artifact history;
  - 73 non-handoff replay evidence files reference the previous source snapshot;
  - newly active non-handoff drivers have no current Rust adapter/evidence yet.

## Next Useful Step

Do not fabricate ledger entries for the stale evidence. Either recover the old
accepted task artifacts from git history and migrate them, or faster for this
dirty experiment, revalidate current-snapshot evidence as each driver is touched.
The handoff lane is the first current lane; the next useful cleanroom-usefulness
step is to make the newly active battle drivers current.
