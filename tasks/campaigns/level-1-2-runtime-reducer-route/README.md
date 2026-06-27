# Campaign Control Files

Start here if you only need orientation.

Real goal:

- Prove and refine a QNT-driven architecture where focused `.qnt` slices define enough reducer-shaped semantics for an independent character/battle runtime to be built from the cleanroom input package.
- The dirty Rust cleanroom is only a rehearsal and evidence target used to save time. Its reducer-spine implementation is not the final goal.
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

The campaign is designed so an orchestrator can resume after losing chat context. If the files and the git worktree disagree, stop and reconcile the files before launching more Ralph workers.
