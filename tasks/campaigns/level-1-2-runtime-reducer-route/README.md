# Campaign Control Files

Start here if you only need orientation.

Real goal:

- Prove and refine a QNT-driven architecture where focused `.qnt` slices define enough reducer-shaped semantics for an independent character/battle runtime to be built from the cleanroom input package.
- The dirty Rust cleanroom is historical rehearsal and diagnostic evidence. The
  current accepted package checkpoint is the fresh target
  `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb` against source package
  `4d196258a51f4264803ff11f7c806c969f0aff2d`.
- The orchestrator leads the campaign, launches Ralph/Codex lane workers, reviews their outputs, merges accepted work, and keeps these files current. A newly resumed agent should not assume it must implement every lane personally.

Primary files:

- `PLAN.md`: human-readable campaign DAG, goals, checkpoints, and operating protocol.
- `STATE.json`: machine-readable current state. Update it after every lane status or integration HEAD change.
- `LANES.json`: lane definitions and driver groups.
- `MERGE_QUEUE.md`: completed reviewed lanes waiting for integration merge.
- `CHECKPOINT_REPORT.md`: checkpoint status and coverage delta log.
- `WORKTREE_LEDGER.md`: worktree ownership and cleanup state.
- `PROMPTS.md`: copyable Ralph worker, reviewer, and fixer briefs.
- `FRESH_CLEANROOM_READINESS.md`: post-CP8 closure evidence, limits, and next work for a future fresh cleanroom run.
- `FRESH_CLEANROOM_DRY_RUN_PLAN.md`: executable first fresh-cleanroom dry-run sequence, boundaries, gates, and required artifacts.
- `FRESH_CLEANROOM_EXPANSION_PLAN.md`: historical fresh-cleanroom expansion plan and acceptance gates.
- `FRESH_EXPANSION_LANES.json`: archived lane list for completed fresh expansion work; not a next-executable queue.
- `NEXT_COMPLETION_AUDIT_LANES.md`: candidate audit-selected follow-up work; not launched lanes.
- `FRESH_SDK_COMPOSITION_ACCEPTANCE.md`: accepted fresh-target evidence for the integrated sheet-to-composed-encounter-to-simple-turn SDK tracer.
- `GOAL_STATUS_AUDIT.md`: requirement-by-requirement status of the active goal, including what is proven and why the global objective remains active.
- `SOURCE_QNT_NEXT_TASKS.md`: historical/current source-QNT queue records plus
  audit-selected future source work, not a list of tasks that currently must be
  added before more target replay.

The campaign is designed so an orchestrator can resume after losing chat context. If the files and the git worktree disagree, stop and reconcile the files before launching more Ralph workers.
