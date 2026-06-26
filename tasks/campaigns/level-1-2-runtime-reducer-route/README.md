# Campaign Control Files

Start here if you only need orientation.

- `PLAN.md`: human-readable campaign DAG, goals, checkpoints, and operating protocol.
- `STATE.json`: machine-readable current state. Update it after every lane status or integration HEAD change.
- `LANES.json`: lane definitions and driver groups.
- `MERGE_QUEUE.md`: completed reviewed lanes waiting for integration merge.
- `CHECKPOINT_REPORT.md`: checkpoint status and coverage delta log.
- `WORKTREE_LEDGER.md`: worktree ownership and cleanup state.
- `PROMPTS.md`: copyable Ralph worker, reviewer, and fixer briefs.

The campaign is designed so an orchestrator can resume after losing chat context. If the files and the git worktree disagree, stop and reconcile the files before launching more Ralph workers.
