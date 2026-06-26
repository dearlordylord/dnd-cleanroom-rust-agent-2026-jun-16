# Level 1-2 Runtime Reducer Route Campaign

## Purpose

Drive the dirty Rust cleanroom toward full level-1/2 runtime coverage through the copied QNT route architecture.

The goal is not to make local replay helpers pass tests. The goal is to make each accepted driver consume the shared reducer-shaped substrate appropriate to its route:

- battle routes use `BattleState`, `start_battle`, `discover_battle_acts`, `resolve_battle_subject`, typed subject/fill variants, `BattleResolutionResult`, and battle-owned durable state;
- component-first routes prove reusable rule components before battle or character routes depend on them;
- catalog/selected-identity routes wait for generic substrates and must not create production identity dispatch;
- replay-refresh-only routes refresh evidence over already-established substrate.

This is a dirty cleanroom rehearsal. It is allowed to reuse the existing Rust cleanroom implementation to save time, but evidence must stay honest about that. Do not claim fresh cleanroom acceptance from this campaign.

## Recovery Instructions

If you are resuming this work after losing conversation context:

1. Read this file first.
2. Read `STATE.json`.
3. Read `LANES.json`.
4. Read `MERGE_QUEUE.md`.
5. Read `WORKTREE_LEDGER.md`.
6. Read `PROMPTS.md`.
7. Confirm the integration worktree status:
   - worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`
   - branch: `ralph/rrconv-19-cleanroom`
   - run `git status --short`
   - run `git rev-parse HEAD`
8. Use the current integration worktree `HEAD` as the base SHA for new lane worktrees, unless `STATE.json` explicitly records a different base policy with a concrete explanation.

## Cleanroom Boundary

Ralph lane workers and reviewers must not read `/workspace/typescript/dnd` or sibling source repositories. They may use only:

- this dirty cleanroom worktree;
- `cleanroom-input/**`;
- `tasks/**`;
- `src/**`;
- repo-local scripts and Cargo tooling.

Production runtime semantics must not branch on authored names, ids, slugs, source headings, provenance sections, page references, or official catalog labels. Fixture/QNT action names are allowed only in adapters, tests, and evidence artifacts.

## Current Baseline

Campaign bootstrap was prepared at integration HEAD:

`6e3ec7c4fff70a28a4ab29cfebeaf9133daec4f0`

Already merged before this campaign file:

- `RRCONV-19A`: battle setup entrypoint.
- `RRCONV-19B`: act discovery contract.
- `RRCONV-19C`: resolution result contract.
- `RRCONV-19D`: turn advancement result.
- `RRCONV-19E`: End Turn subject resolution.
- `RRCONV-19F`: route events from reducer results.
- `RRCONV-19G`: subject continuation lifecycle.
- `T001` through `T005`: dirty diagnostic reducer-spine driver evidence.

Coverage at bootstrap:

- canonical level denominator: `level-1-5-cleanroom-route-v1`;
- level-1/2 ready queue: `97` driver files;
- canonical in-scope obligations: `668`;
- accepted unique selected drivers in ledger: `7`;
- accepted obligations for those drivers: `52`;
- current driver coverage: `7 / 97 = 7.2%`;
- current obligation coverage: `52 / 668 = 7.8%`.

The route-assignment rows currently sum to `699` obligations before branch-scope exclusions. Use `668` as the canonical in-scope denominator unless the copied inventory is intentionally refreshed and this campaign state is updated.

## Campaign Model

This is one Ralph campaign made of checkpointed DAG phases. It is not one giant implementation task.

Each lane is an atomic Ralph implementation unit with:

- one route task id from `cleanroom-input/branch-coverage/reducer-route-inventory.json`;
- a bounded driver set;
- one clear route class or substrate family;
- its own worktree and branch;
- read-only review before merge;
- integration checks after merge.

Recommended maximum for a single lane:

- at most `10` drivers, or
- at most about `70` branch obligations,
- unless the lane exists specifically to split a larger source-defined route task.

Lanes that exceed that size in the inventory are split in `LANES.json`.

## Checkpoints

### CP0: Bootstrap

Status: complete.

Purpose:

- freeze denominator and baseline;
- record completed RRCONV-19 substrate work;
- create durable campaign plan/state files.

Exit criteria:

- campaign files exist;
- integration branch is clean;
- final pre-campaign checks from RRCONV-19 are known to have passed.

### CP1: Battle Reducer Core Expansion

Purpose:

Expand from focused diagnostic drivers to the next battle reducer subjects that should reuse the same reducer-spine substrate.

Initial lanes:

- `L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE`
- `L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES`
- `L15-RR06-BATTLE-SPELL-EFFECT-ROUTES`

Parallelism:

- up to 2 lanes at once.
- Prefer starting `RR03` and `RR05` first.
- Start `RR06` after the first merge if either earlier lane changes shared spell-effect or action-economy substrate.

Exit criteria:

- all CP1 lanes merged;
- `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node scripts/check-cleanroom-harness.cjs`, and `git diff --check` pass on integration;
- `CHECKPOINT_REPORT.md` updated with coverage delta.

### CP2: Rule-Core Component Connectors

Purpose:

Prove reusable rule-core components before battle/character lanes depend on them.

Lane:

- `L15-RR04-RULE-CORE-COMPONENT-CONNECTORS`

Parallelism:

- split into sublanes before execution because it has `9` drivers and `140` obligations.
- Maximum recommended split: 3 sublanes grouped by shared Rust module/write scope.

Exit criteria:

- rule-core component evidence is merged;
- no battle adapter imports component witness/protocol state as production runtime state.

### CP3: Character Creation, Sheet, And Handoff

Purpose:

Establish character-side and handoff substrates so battle routes do not invent character state locally.

Lanes:

- `L15-RR08-CHARACTER-CREATION-ROUTES`
- `L15-RR09-CHARACTER-SHEET-ROUTES`
- `L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES`

Parallelism:

- up to 3 lanes if write scopes are disjoint;
- merge order: creation, sheet, handoff.

Exit criteria:

- character and sheet durable state owners are recorded;
- handoff routes consume those owners rather than duplicate projections.

### CP4: Feature And Catalog Substrates

Purpose:

Build generic feature/species/spell/metamagic substrates before selected-identity/catalog drivers are accepted.

Lanes:

- `L15-RR07-BATTLE-FEATURE-SUBSTRATE-ROUTES`
- `L15-RR07-FU01-LEVEL1-SPELL-IDENTITY-SUBSTRATES`
- `L15-RR07-FU02-SPECIES-PASSIVE-TRAIT-SUBSTRATES`
- `L15-RR07-FU03-CONDITION-AND-ROLL-MODIFIER-FEATURE-SUBSTRATES`
- `L15-RR07-FU04-ZERO-HP-STABILIZATION-SUBSTRATE`
- `L15-RR07-FU05-FEATURE-MOVEMENT-AND-FORM-SUBSTRATES`
- `L15-RR07-FU06-WEAPON-MASTERY-PROPERTY-SUBSTRATES`
- `L15-RR07-FU07-DRAGONBORN-BREATH-WEAPON-SUBSTRATE`
- `L15-RR07-FU08-METAMAGIC-GOVERNOR-AND-OPTION-SUBSTRATES`
- `L15-RR07-FU09-INNATE-SPELL-BENEFIT-FEATURE-SUBSTRATE`

Parallelism:

- up to 4 lanes after CP2 and CP3 are merged;
- split `FU01` and `FU08` before execution because they exceed the preferred lane size or contain blocker/corpus work.

Exit criteria:

- selected-identity behavior is admitted by shape/substrate facts, not identity dispatch;
- blocker lanes either resolve source-QNT/corpus blockers or record concrete blocker evidence.

### CP5: Remaining Battle Families

Purpose:

Cover remaining battle runtime families that depend on CP1-CP4 substrate.

Lanes:

- `L15-RR15-AFTER-HIT-RIDER-OWNER-SPLIT`
- `L15-RR16-CHAINED-ATTACK-PROCEDURE-ROUTES`
- `L15-RR17-WEAPON-HOSTED-RIDER-ROUTES`
- `L15-RR18-BATTLE-ACTIVE-EFFECT-LIFECYCLE-ROUTES`
- `L15-RR19-BATTLE-REACTION-INTERRUPT-ROUTES`
- `L15-RR20-BATTLE-COMPANION-OBJECT-BOUNDARY-ROUTES`
- `L15-RR21-BATTLE-ABILITY-SEARCH-CHOICE-ROUTES`
- `L15-RR22-BATTLE-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES`

Parallelism:

- up to 4 lanes;
- keep reaction/interrupt and active-effect lanes serialized if they touch the same continuation/interrupt stack state.

Exit criteria:

- remaining battle route groups covered;
- no driver-local replay islands remain for these route groups.

### CP6: Closure Sweep

Purpose:

Reconcile coverage and remove campaign bookkeeping ambiguity.

Required work:

- recompute covered drivers and obligations from `tasks/RUN_LEDGER.json`;
- verify every in-scope driver is covered, explicitly blocked, or out-of-scope by copied inventory;
- run full integration verification;
- update `CHECKPOINT_REPORT.md`;
- mark all merged worktrees removable in `WORKTREE_LEDGER.md`;
- leave final handback in `tasks/HANDBACK.md` or a campaign-specific final report.

Exit criteria:

- `STATE.json.status` is `complete` or `blocked`;
- every blocker has a concrete source QNT/corpus, target implementation, or scope reason.

## Standard Lane Lifecycle

1. Mark lane `ready` -> `running` in `STATE.json`.
2. Resolve the base SHA with `git rev-parse HEAD` in the integration worktree.
3. Create a worktree from that base SHA.
4. Spawn Ralph worker with:
   - lane id;
   - base SHA;
   - cleanroom boundary;
   - allowed write scope;
   - selected drivers;
   - verification commands.
5. Worker commits lane.
6. Mark lane `implemented`.
7. Spawn read-only reviewer.
8. If findings exist, spawn fixer in same lane worktree and amend or add a fix commit.
9. Mark lane `reviewed`.
10. Merge into integration branch with `--no-ff`.
11. Run integration gates.
12. Mark lane `merged`.
13. Update coverage in `CHECKPOINT_REPORT.md`.
14. Mark lane worktree `removable` only after integration checks pass.

## Standard Verification

Lane verification should include:

- `cargo fmt --check`
- focused tests for the selected drivers
- `cargo test reducer_spine_contract_adapter_replays_all_branches` when battle reducer substrate is touched
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `node scripts/check-cleanroom-harness.cjs`
- `git diff --check <base>...HEAD`

Integration verification after each merge:

- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `node scripts/check-cleanroom-harness.cjs`
- `git diff --check HEAD~1...HEAD`

## Merge Discipline

- Do not rebase lane branches unless explicitly repairing a branch-base mismatch before implementation.
- Every lane worker must run the task-provided base check.
- The orchestrator owns merge order and branch repair.
- If two lanes touch the same production module or harness artifact in incompatible ways, merge one, rerun integration checks, then start or rebase/recreate the dependent lane from the new integration head.

## What Counts As Done

A driver is done only when:

- target replay evidence exists under `tasks/target-replay-evidence/`;
- `tasks/RUN_LEDGER.json` records accepted evidence refs;
- `tasks/VALIDATION_REPORT.md` reports the exact driver obligations as covered;
- engine-depth and state-owner artifacts are honest;
- focused Rust tests pass;
- integration checks pass after merge.

Diagnostic tests alone do not close branch coverage.
