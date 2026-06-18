# Blockers

## B001: Dirty All-Lane Refresh Requires Current-Snapshot Replays

- Task id: dirty-refresh-all-5-lanes
- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Blocker class: `target-implementation`
- Affected drivers: all active `level-1-2-full` drivers without current-snapshot replay evidence
- Missing target capability: current-snapshot `tasks/RUN_LEDGER.json`, `tasks/history/<taskId>/*.json`, and replay evidence for the 96-driver active queue
- Corpus files consulted:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `tasks/ACTIVE_WORK.json`
  - `tasks/LEVEL_1_2_SCOPE.md`
  - `tasks/target-replay-evidence/*.json`
- Exact target gap: the handoff lane plus T060/T062/T063/T064/T074/T079/T080 now have current replay evidence, but 68 replay evidence files still reference source commit `04249edf345a7752de2f1551dd3d509a2fffc160` and source branch inventory SHA `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`, while the refreshed snapshot requires source commit `829aee6441d76a921c9d9c14a0d0221062975334` and inventory SHA `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`.
- Next action: re-run or regenerate replay evidence under the refreshed source snapshot for the remaining non-handoff drivers and append accepted tasks to the new run-ledger/history artifact model.

## B002: Newly Active Non-Handoff Drivers Have No Rust Replay Adapter Yet

- Task id: dirty-refresh-all-5-lanes
- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Blocker class: `target-implementation`
- Affected drivers: 18 non-handoff drivers newly added to the active queue by the refreshed source inventory
- Missing target capability: Rust production/API support where needed, quarantined `src/qnt_adapters/*` modules, current replay evidence, and ledger/history entries for those drivers
- Corpus files consulted:
  - `tasks/ACTIVE_WORK.json`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/**`
- Exact target gap: handoff's 4 newly active drivers now have dirty Rust adapters and current evidence. The remaining newly active drivers are creation, sheet, battle, and rules-core drivers listed in `tasks/DIRTY_REHEARSAL_STATUS.md`.

Record a blocker only when an allowed input is insufficient for a behavior a
task requires or when the target implementation cannot yet satisfy a replayable
source branch.

Each entry states:

- task id;
- manifest source commit SHA;
- source branch inventory SHA;
- blocker class: `source-qnt-corpus`, `source-scope`, or
  `target-implementation`;
- affected driver and branch action when applicable;
- missing fact or failing target capability;
- corpus files consulted;
- exact question the corpus cannot answer, or exact target gap that remains.
